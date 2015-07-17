extern crate rustc_serialize;

pub mod tcp;
pub mod message;


use tcp::*;
use message::*;
use std::io::{ Result };
use std::net::{ SocketAddr };

pub struct Bubbles;

impl Bubbles {

    pub fn listen_on( addr: SocketAddr ) -> Option<BubbleIter> {
        match listen_on( addr ) {
            Some( pl ) => Some( BubbleIter { inner: pl } ),
            None => None
        }
    }

    pub fn connect_to( addr: SocketAddr ) -> Option<Bubble> {
        match connect_to( addr ) {
            Some( p ) => Some( Bubble { inner: p } ),
            None => None
        }
    }
}

pub struct Bubble {
    inner: TcpPortal
}

impl Bubble {
    pub fn send_message( &mut self, message: Message ) -> Result<()> {
        self.inner.send_message( message )
    }

    pub fn rcv_message( &mut self ) -> Option<Message> {
        self.inner.rcv_message() 
    }
}

pub struct BubbleIter {
    inner: PortalListener
}

impl Iterator for BubbleIter {
    
    type Item = Bubble;

    fn next( &mut self ) -> Option<Bubble> {
        match self.inner.accept() {
            Some( p ) => Some( Bubble{ inner: p } ),
            None => None
        }
    }
}
