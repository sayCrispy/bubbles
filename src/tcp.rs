use std::net::{ TcpStream, TcpListener, SocketAddr };
use std::io::{ BufReader, Lines, LineWriter, BufRead, Write, Result };
use std::error::Error;
use message::*;

pub struct TcpPortal  {
    pub socket_addr: SocketAddr,

    input: Lines<BufReader<TcpStream>>,
    output: LineWriter<TcpStream>,
}

impl TcpPortal {
    fn new ( stream: TcpStream, s_a: SocketAddr ) -> Result<TcpPortal> {
        
        let stream_clone;

        match stream.try_clone() {
            Ok( s ) => stream_clone = s,
            Err( e ) => return Err( e )
        }

        Ok ( TcpPortal {
            socket_addr: s_a,

            input: BufReader::new( stream ).lines(),
            output: LineWriter::new( stream_clone ),
        } )
    }

    pub fn send_message( &mut self, message: Message ) -> Result<()> {
        let buf: String;
        match message.to_json()  {
            Ok( s ) => buf = s + "\n",
            Err( e ) => return Err( e ),
        }
        match self.output.write( buf.as_bytes() ) {
            Ok( _ ) => Ok( () ),
            Err( e ) => Err( e ),
        }
    }

    pub fn rcv_message( &mut self ) -> Option<Message> {
        match self.input.next() {
            Some( r ) => {
                match r {
                    Ok( s ) => match Message::from_json( s ) {
                        Ok( m ) => Some( m ),
                        Err( e ) => {
                            println!( "{}", e.description() );
                            None
                        }
                    },
                    Err( e ) => {
                        println!( "{}", e.description() );
                        None
                    }
                }
            }
            None => None
        }
    }
}

pub struct PortalListener {
    inner: TcpListener,
}

impl PortalListener {
    
    pub fn accept( &self ) -> Option<TcpPortal> {

        let stream: TcpStream;
        let addr: SocketAddr;

        match self.inner.accept() {
            Ok( ( s, a ) ) => {
                stream = s;
                addr = a;
            }
            Err( e ) => {
                println!( "{}", e.description() );     
                return None
            }
        }

        match TcpPortal::new( stream, addr ) {
            Ok( p ) => Some( p ),
            Err( e ) => {
                println!( "{}", e.description() );     
                return None
            }
        }
    }
}

pub fn listen_on( addr: SocketAddr ) -> Option<PortalListener> {

    match TcpListener::bind( addr ) {
        Ok( l ) => Some( PortalListener { inner: l } ),
        Err( e ) => {
            println!( "{}",  e.description() );
            None
        }
    }
}

pub fn connect_to( addr: SocketAddr ) -> Option<TcpPortal> {

    let stream: TcpStream;

    match TcpStream::connect( addr ) {
        Ok( s ) => stream = s,
        Err( e ) => {
            println!( "{}",  e.description() );
            return None;
        }
    }

    match TcpPortal::new( stream, addr ) {
        Ok( p ) => Some( p ),
        Err( e ) => {
            println!( "{}",  e.description() );
            None
        }
    }
}
