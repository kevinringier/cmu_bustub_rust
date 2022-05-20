use std::fs::{File, OpenOptions};
use std::io::{Read, ErrorKind, Seek, SeekFrom, Write};
use std::sync::Mutex;

use crate::include::common::{PAGE_SIZE, PageId};

/// DiskManager is responsible for reading and writing data from disk.
pub struct DiskManager {
    latched_log_io: Mutex<File>,
    log_name: String,
    latched_db_io: Mutex<File>,
    file_name: String,
    num_flushes: isize,
    num_writes: isize,
    flush_log: bool,
    // used for checking non-blocking flushing, not sure what this means exactly.  
    //flush_log_f: std::future::Future<Void>,
    // buffer used appears to buffer the log contents right before writing and flushing to the log file
    //buffer_used: String,
}

impl DiskManager {
    pub fn new(db_file: String) -> DiskManager {

        // get log file name
        let log_name = match db_file.rfind(".") {
            Some(index) => String::from(&db_file[..index]) + ".log",
            None => panic!("wrong file format")
        };

        // create/open log file
        let latched_log_io: Mutex<File> = Mutex::new(DiskManager::get_log_file(&log_name));

        // create/open db file
        let latched_db_io: Mutex<File> = Mutex::new(DiskManager::get_db_file(&db_file));
        
        DiskManager {
            latched_log_io: latched_log_io,
            log_name: log_name,
            latched_db_io: latched_db_io,
            file_name: db_file,
            num_flushes: 0,
            num_writes: 0,
            flush_log: false,
        }
    }

    pub fn shutdown(&self) {
        // I don't think we need this. Rust should close the file when the last owner releases it's reference.  
        panic!("not implemented");
    }

    /// Write the contents of the specified page into the disk file
    // TODO: should page_data be a string parameter?
    pub fn write_page(&mut self, page_id: PageId, page_data: &[u8]) {
        let mut db_io = self.latched_db_io.lock().unwrap();
        let offset: u64 = (page_id.as_int * PAGE_SIZE as isize) as u64;

        match db_io.seek(SeekFrom::Start(offset)) {
            Ok(_) => match db_io.write(page_data) {
                Ok(_) => self.num_writes += 1,
                Err(e) => panic!("I/O error while writing: {}", e)
            },
            Err(e) => panic!("error while seeking: {}", e)
        }
        
        // I don't think we need to flush here since we are not using a buffer and writing directly to the file. 
        // Mutex unlocked when MutexGuard is dropped at end of scope.
    }

    /// Read the contents of the specified page into the given memory area
    // How many logs are we returning?  Just one record, multiple records?
    // this question determines how we retrieve logs
    // read exact may not work the way we want.  may need to implemented a 
    // read buffer internally to retrieve the desired amount of logs then
    // fill the associated buffer with the desired logs. this seems expensive
    // think more about this.  
    pub fn read_page(&mut self, page_id: PageId, page_data: &mut [u8]) {
        let mut db_io = self.latched_db_io.lock().unwrap();
        let offset: u64 = (page_id.as_int * PAGE_SIZE as isize) as u64;

        if offset > DiskManager::get_file_size(&db_io) {
            panic!("I/O error reading past end of file");
        } else {
            match db_io.seek(SeekFrom::Start(offset)) {
                Ok(_) =>  match db_io.read_exact(page_data) {
                    Ok(_) => (),
                    
                    Err(e) => match e.kind() {
                        // TODO: will need to fix.  Leaving for now.
                        // c++
                        //// if file ends before reading PAGE_SIZE
                        // int read_count = db_io_.gcount();
                        // if (read_count < PAGE_SIZE) {
                        // LOG_DEBUG("Read less than a page");
                        // db_io_.clear();
                        // // std::cerr << "Read less than a page" << std::endl;
                        // memset(page_data + read_count, 0, PAGE_SIZE - read_count);
                        ErrorKind::UnexpectedEof => panic!("read less than a page"),
                        _ => panic!("I/O error reading page_id -> {} into buffer: {}", page_id.as_int, e)
                    }
                },
                Err(_) => ()
            }
        }
    }

    /// Write the contents of the log into disk file
    /// Only return when sync is done, and only perform sequence write
    pub fn write_log(&mut self, log_data: &[u8]) {
        // why is the log_file not protected by mutex?
        // it seems like the CMU implementation checks for non-blocking flushing. 
        // does that imply flushing in c++ blocks?
        // I'm going to implement with a mutex. I can't understand why they chose
        // to use a future that presumably checks for non-blocking flushing.
        // This may imply that flushing blocks, or maybe this helps with sequence writes.
        // but I will explicitly block to avoid confusion.

        panic!("after using api in buffer pool manager, does the cmu implementation make sense?");

        let mut log_io = self.latched_log_io.lock().unwrap();
        match log_io.write(log_data) {
            // TODO: refactor newline write to log.  Should I use newline character between logs?
            Ok(_) => {
                self.num_flushes += 1;
                let newline = String::from("\n");
                let _ = log_io.write(newline.as_bytes());
            },
            Err(e) => panic!("I/O error while writing log: {}", e)
        }
    }

    /// Read the contents of the log into the given memory area
    /// Always read from the beginning and perform sequence read
    /// return false means already reach the end
    pub fn read_log(&self, log_data: &mut Vec<u8>) {
        let mut log_io = self.latched_log_io.lock().unwrap();
        
        match log_io.read_to_end(log_data) {
            Ok(num_bytes_read) => (), // implement information logging to the console
            Err(e) => panic!("{}", e)
        }
    }

    /// Returns number of flushes made so far
    pub fn get_num_flushes(&self) -> isize { self.num_flushes }

    /// Returns number of writes made so far
    pub fn get_num_writes(&self) -> isize { self.num_writes }

    /// Returns true if the log is currently being flushed
    pub fn get_flush_state(&self) -> bool { self.flush_log }

    fn get_log_file(log_name: &String) -> File {
        match OpenOptions::new().write(true).append(true).read(true).open(&log_name) {
            Ok(result) => result,
            Err(_) => {
                // CMU bustub implementation opens the file in truncate mode,
                // then closes the file, then reopens the file with desired options
                match OpenOptions::new().write(true).create_new(true).append(true).truncate(true).read(true).open(&log_name) {
                    Ok(result) => result,
                    Err(e) => panic!("can't open dblog file: {}", e),
                }
            }
        }
    }

    fn get_db_file(db_name: &String) -> File {
        match OpenOptions::new().read(true).write(true).open(&db_name) {
            Ok(result) => result,
            Err(_) => {
                // CMU bustub implementation opens the file in truncate mode,
                // then closes the file, then reopens the file with desired options
                match OpenOptions::new().write(true).create_new(true).truncate(true).read(true).open(&db_name) {
                    Ok(result) => result,
                    Err(e) => panic!("can't open db file: {}", e)
                }
            }
        }
    }

    fn get_file_size(file: &File) -> u64 {
        match file.metadata() {
            Ok(metadata) => metadata.len(),
            Err(e) => panic!("error retrieving file metadata: {}", e)
        }
    }
}

mod disk_manager_tests {
    use super::DiskManager;
    use crate::include::common::{PageId};

    #[test]
    #[ignore] // need to clear files in between tests
    fn get_log_file_opens_new_file() {
        DiskManager::get_log_file(&String::from("db.log"));
        assert_eq!(1,1);
    }

    #[test]
    #[ignore] // need to clear files in between tests
    fn test_write_to_db_file() {
        let mut dm = DiskManager::new(String::from("test.db"));

        let page_id = PageId::new(0);
        let data = String::from("test values to be written");
        dm.write_page(page_id, data.as_bytes());

        let page_id_2 = PageId::new(2);
        let data2 = String::from("more test values");
        dm.write_page(page_id_2, data2.as_bytes());

        let page_id_1 = PageId::new(1);
        let data3 = String::from("this should be in the middle");
        dm.write_page(page_id_1, data3.as_bytes());
    }

    #[test]
    #[ignore]
    fn test_write_to_log_file() {
        let mut dm = DiskManager::new(String::from("test.db"));

        let log_data_1 = String::from("this is the first log message");

        dm.write_log(log_data_1.as_bytes());

        let log_data_2 = String::from("second log");

        dm.write_log(log_data_2.as_bytes());
    }

    #[test]
    fn test_read_log_into_memory() {
        let mut dm = DiskManager::new(String::from("test.db"));
        let mut buffer: Vec<u8> = Vec::new();

        dm.read_log(&mut buffer);

        match String::from_utf8(buffer) {
            Ok(result) => println!("{}", result),
            Err(e) => println!("{}", e)
        }
        
    }
}