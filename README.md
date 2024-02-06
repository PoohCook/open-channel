# Open Channel

Simple project to support exchange of message structures via a serial interface.

The destination, and source of the serial data is assumed to be an STM32F405 with limited resources

therefore high level communications like JSON are impractical. The STM code (which already exists)

exchanges data in packets of bytes via an SPI interface using a simple message type followed by the

correct number of bytes for that message..

The names in here are bit silly but so is the project...  :-)

It was actually done as a challenge to see how far I could get in implmenting something that allows
reception of commands whos length and content are not know until run time...  IOW, breaking fundemental
rules of Rust....
