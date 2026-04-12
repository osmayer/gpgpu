use std::fs::{self, File};
use clap::Parser;
use object::{self, Object, ObjectSection};
use byteorder::{ByteOrder, LittleEndian};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PrivilegeValue {
    USER,
    KERNEL
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SegmentMetadata {
    pub base_addr:    u32,
    pub max_size:     u32,
    pub name:         &'static str,
    pub privilege:    PrivilegeValue,
    pub executable:   bool
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Segment {
    pub metadata: SegmentMetadata,
    pub values:   Vec<u8>
}


const USER_TEXT_START: u32   = 0x00400000;
const USER_DATA_START: u32   = 0x10000000;
const STACK_END: u32         = 0x7ff00000;
const STACK_SIZE: u32        = 1 * 1024 * 1024;
const STACK_START: u32       = STACK_END - STACK_SIZE;
const KERNEL_TEXT_START: u32 = 0x80000000;
const KERNEL_DATA_START: u32 = 0x90000000;


const IMAGE_STRUCTURE: [ SegmentMetadata; 5 ] = [
    SegmentMetadata {
        base_addr:  USER_TEXT_START,
        max_size:   USER_DATA_START - USER_TEXT_START,
        name:       "text",
        privilege:  PrivilegeValue::USER,
        executable: true
    },
    SegmentMetadata {
        base_addr:  USER_DATA_START,
        max_size:   STACK_START - USER_DATA_START,
        name:       "data",
        privilege:  PrivilegeValue::USER,
        executable: false
    },
    SegmentMetadata {
        base_addr:  STACK_START,
        max_size:   STACK_SIZE,
        name:       "stack",
        privilege:  PrivilegeValue::USER,
        executable: false
    },
    SegmentMetadata {
        base_addr:  KERNEL_TEXT_START,
        max_size:   KERNEL_DATA_START - KERNEL_TEXT_START,
        name:       "kdata",
        privilege:  PrivilegeValue::KERNEL,
        executable: true
    },
    SegmentMetadata {
        base_addr:  KERNEL_DATA_START,
        max_size:   std::u32::MAX - KERNEL_DATA_START,
        name:       "kdata",
        privilege:  PrivilegeValue::KERNEL,
        executable: false
    }
]; 


impl Segment {
    fn new(segment_data: Vec<u8>, metadata: SegmentMetadata) -> Self {
        Segment { 
            metadata, 
            values: segment_data 
        }
    }
}


pub fn file_to_image (obj_file: &object::File) -> Vec<Segment> {
    let mut results = vec![];
    for segment in IMAGE_STRUCTURE {
        if let Some(section_obj) = obj_file.section_by_name(format!(".{}", segment.name).as_str()) {
            let section_data: Vec<u8> = section_obj
                                            .data()
                                            .expect("Illegal Data")
                                            .into();
            
            results.push(Segment::new(section_data, segment));

        }
    }
    results
}

pub fn get_u32 (memory_map: &Vec<Segment>, req_addr: u32) -> Option<u32> {
    for segment in memory_map {
        let segment_start = segment.metadata.base_addr as usize;
        let segment_end   = (segment.metadata.base_addr + segment.metadata.max_size) as usize;
        let req_start_addr= req_addr as usize;
        let req_end_addr  = (req_start_addr + 4) as usize;

        if req_start_addr >= segment_start && req_end_addr < segment_end {
            let effective_start = req_start_addr - segment_start;
            let effective_end   = effective_start + 4; 
            
            if effective_end > segment.values.len() {
                println!("Too long!!");
                return None;
            }

            return Some(LittleEndian::read_u32(&segment.values[effective_start..effective_end]));
        }
    }
    None
}


pub fn set_u32 (memory_map: &mut Vec<Segment>, req_addr: u32, new_value: u32) -> Option<u32> {
    for segment in memory_map {
        let segment_start = segment.metadata.base_addr as usize;
        let segment_end   = (segment.metadata.base_addr + segment.metadata.max_size) as usize;
        let req_start_addr= req_addr  as usize;
        let req_end_addr  = (req_start_addr + 4) as usize;

        if req_start_addr >= segment_start && req_end_addr < segment_end {
            let effective_start = req_start_addr - segment_start;
            let effective_end   = effective_start + 4; 

            if effective_end >= segment.values.len() {
                segment.values.resize(effective_end, 0);
            }

            let prev_val               = LittleEndian::read_u32(&mut segment.values[effective_start..effective_end]);
            LittleEndian::write_u32(&mut segment.values[effective_start..effective_end], new_value);
            return Some(prev_val);
        }
    }
    None
}