use rustc_serialize::json;
use std::io::Error as E;
use std::io::ErrorKind as EK;
use std::error::Error;

pub enum M {
    DV { dv: DoubleVec },
    PING,
}

pub struct Message {
    inner: M,
}

impl Message {
    pub fn new( message: M ) -> Message {
        Message {
            inner: message,
        }
    }
 
    pub fn to_json ( self ) -> Result<String, E> {
        let inner = self.inner;
        match inner {
            M::PING => Ok( "0".to_string() ),
            M::DV { dv } => match dv.to_json() {
                Ok( s ) => {
                    let mut ms = s;
                    ms.push( '1' );
                    return Ok( ms );
                },
                Err( e ) => Err( e ),
            },
       }
    }

    pub fn from_json ( s: String ) -> Result<Message, E> {
        let mut ms = s;
        match ms.pop() {
            Some( c ) =>  {
                match c {
                    '0' => Ok( Message::new( M::PING ) ),
                    '1' => match DoubleVec::from_json( ms ) {
                        Ok ( dv ) => Ok ( Message::new( M::DV { dv: dv } ) ),
                        Err ( e ) => Err ( e ),
                    },
                    _ => Err( E::new( EK::Other, "unknown message type" ) ),

                }
            },
            None => Err( E::new( EK::Other, "empty message" ) ),        
        }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DoubleVec {
    string_vec: Vec<String>,
    i64_vec: Vec<i64>,
}

impl DoubleVec {

    fn to_json( &self ) -> Result<String, E> {
        match json::encode( &self ) {
            Ok( s ) => Ok( s ),
            Err( e ) => Err( E::new( EK::Other, e.description() ) ),
        }
    }

    fn from_json( s: String ) -> Result<DoubleVec, E> {
        match json::decode( &s ) {
            Ok( dv ) => Ok ( dv ),
            Err( e ) => Err( E::new( EK::Other, e.description() ) ),
        }
    }
}
