//! Provides a trait for receiving data frames and messages.
//!
//! Also provides iterators over data frames and messages.
//! See the `ws` module documentation for more information.

use std::io::Read;
use ws::Message;
use ws::dataframe::DataFrame;
use result::WebSocketResult;
use uuid::Uuid;

/// A trait for receiving data frames and messages.
pub trait Receiver: Sized {
	/// The type of dataframe that incoming messages will be serialized to.
	type F: DataFrame;

	/// The type of message that incoming messages will be serialized to.
	type M: Message;

	///	Gets the UUID of a receiver
	fn uuid(&self) -> Uuid;

	/// Reads a single data frame from this receiver.
	fn recv_dataframe<R>(&mut self, reader: &mut R, uuid: Uuid) -> WebSocketResult<Self::F> where R: Read;

	/// Returns the data frames that constitute one message.
	fn recv_message_dataframes<R>(&mut self, reader: &mut R) -> WebSocketResult<Vec<Self::F>>
		where R: Read;

	/// Returns an iterator over incoming data frames.
	fn incoming_dataframes<'a, R>(&'a mut self, reader: &'a mut R) -> DataFrameIterator<'a, Self, R>
		where R: Read
	{
		DataFrameIterator {
			reader: reader,
			inner: self,
		}
	}

	/// Reads a single message from this receiver.
	fn recv_message<'m, R>(&mut self, reader: &mut R) -> WebSocketResult<Self::M>
		where R: Read
	{
		let dataframes = self.recv_message_dataframes(reader)?;
		Self::M::from_dataframes(dataframes)
	}

	/// Returns an iterator over incoming messages.
	fn incoming_messages<'a, R>(&'a mut self, reader: &'a mut R) -> MessageIterator<'a, Self, R>
		where R: Read
	{
		MessageIterator {
			reader: reader,
			inner: self,
		}
	}
}

/// An iterator over data frames from a Receiver.
pub struct DataFrameIterator<'a, Recv, R>
	where Recv: 'a + Receiver,
	      R: 'a + Read
{
	reader: &'a mut R,
	inner: &'a mut Recv,
}

impl<'a, Recv, R> Iterator for DataFrameIterator<'a, Recv, R>
	where Recv: 'a + Receiver,
	      R: Read
{
	type Item = WebSocketResult<Recv::F>;

	/// Get the next data frame from the receiver. Always returns `Some`.
	fn next(&mut self) -> Option<WebSocketResult<Recv::F>> {
		let uuid = self.inner.uuid();
		Some(self.inner.recv_dataframe(self.reader, uuid))
	}
}

/// An iterator over messages from a Receiver.
pub struct MessageIterator<'a, Recv, R>
	where Recv: 'a + Receiver,
	      R: 'a + Read
{
	reader: &'a mut R,
	inner: &'a mut Recv,
}

impl<'a, Recv, R> Iterator for MessageIterator<'a, Recv, R>
	where Recv: 'a + Receiver,
	      R: Read
{
	type Item = WebSocketResult<Recv::M>;

	/// Get the next message from the receiver. Always returns `Some`.
	fn next(&mut self) -> Option<WebSocketResult<Recv::M>> {
		Some(self.inner.recv_message(self.reader))
	}
}
