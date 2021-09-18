use super::error;

use dr;
use super::decoder;

use std::mem;
use std::str;

#[derive(Debug)]
pub enum State {
    /// Parsing completed
    Complete,
    /// Consumer requested to stop parse
    ConsumerStopRequested,
    /// Consumer errored out with the given error
    ConsumerError(Box<error::Error>),
    HeaderIncorrect,
    ChunkIncorrect,
    DecoderError(error::Error),
}

pub enum Action {
    Continue,
    Stop,
    Error(Box<error::Error>),
}

pub trait Consumer {
    fn initialize(&mut self) -> Action;
    fn finalize(&mut self) -> Action;

    fn consume_header(&mut self, header: &dr::DxbcHeader) -> Action { Action::Continue }
    fn consume_rdef(&mut self, rdef: &dr::RdefChunk) -> Action { Action::Continue }
    fn consume_isgn(&mut self, isgn: &dr::IOsgnChunk) -> Action { Action::Continue }
    fn consume_osgn(&mut self, osgn: &dr::IOsgnChunk) -> Action { Action::Continue }
    fn consume_shex(&mut self, osgn: &dr::ShexHeader) -> Action { Action::Continue }
    fn consume_stat(&mut self, osgn: &dr::IStatChunk) -> Action { Action::Continue }
    fn consume_instruction(&mut self, offset: u32, instruction: dr::SparseInstruction) -> Action { Action::Continue }
}

fn try_consume(action: Action) -> Result<(), State> {
    match action {
        Action::Continue => Ok(()),
        Action::Stop => Err(State::ConsumerStopRequested),
        Action::Error(err) => Err(State::ConsumerError(err)),
    }
}

pub struct Parser<'c, 'd> {
    decoder: decoder::Decoder<'d>,
    consumer: &'c mut Consumer,
}

impl<'c, 'd> Parser<'c, 'd> {
    pub fn new(binary: &'d [u8], consumer: &'c mut Consumer) -> Self {
        Parser {
            decoder: decoder::Decoder::new(binary),
            consumer,
        }
    }

    pub fn parse(&mut self) -> Result<(), State> {
        try_consume(self.consumer.initialize())?;

        let header = self.parse_header()?;
        try_consume(self.consumer.consume_header(header))?;

        let chunk_offsets = self.decoder.words(header.chunk_count as usize);

        for &chunk_offset in chunk_offsets {
            self.decoder.seek_mut(chunk_offset as usize);
            let fourcc = self.decoder.bytes(4);
            let chunk_length = self.decoder.read_u32();

            let mut decoder = self.decoder.scoped_decoder(chunk_length as usize);

            match fourcc {
                b"RDEF" => {
                    let rdef = dr::RdefChunk::parse(&mut decoder)?;
                    try_consume(self.consumer.consume_rdef(&rdef))?;
                },
                b"ISGN" => {
                    let isgn = dr::IOsgnChunk::parse(&mut decoder)?;
                    try_consume(self.consumer.consume_isgn(&isgn))?;
                },
                b"OSGN" => {
                    let osgn = dr::IOsgnChunk::parse(&mut decoder)?;
                    try_consume(self.consumer.consume_osgn(&osgn))?;
                },
                b"SHEX" => {
                    let shex = dr::ShexHeader::parse(&mut decoder)?;
                    try_consume(self.consumer.consume_shex(&shex))?;

                    let mut decoder = decoder.scoped_decoder(shex.instruction_length as usize * 4);

                    while !decoder.eof() {
                        let offset = decoder.get_offset();
                        let instruction = dr::SparseInstruction::parse(&mut decoder);

                        try_consume(self.consumer.consume_instruction(offset as u32, instruction))?;
                    }
                },
                b"STAT" => {
                    let stat = dr::IStatChunk::parse(&mut decoder)?;
                    try_consume(self.consumer.consume_stat(&stat))?;
                }
                _ => {
                    eprintln!(
                        "{}: Incorrect or unimplemented chunk type '{}'",
                        chunk_offset,
                        unsafe {
                            str::from_utf8_unchecked(fourcc)
                        },
                    );
                }
            }
        }

        try_consume(self.consumer.finalize())?;

        Ok(())
    }

    fn parse_header(&mut self) -> Result<&'d dr::DxbcHeader, State> {
        let bytes = self.decoder.bytes(mem::size_of::<dr::DxbcHeader>());

        let header: &'d dr::DxbcHeader = unsafe {
            mem::transmute(bytes.as_ptr())
        };

        if header.magic == *b"DXBC" {
            Ok(header)
        } else {
            Err(State::HeaderIncorrect)
        }
    }
}
