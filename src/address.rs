use crate :: { import::*, *, thread_safe::BoxRecipient };


/// An address that allows you to send messages to an actor
///
pub trait Address<A, M> : Clone + Recipient<M>

	where A: Actor + Handler<M>,
	      M: Message           ,
{
	fn recipient( &self ) -> BoxRecipient<M>;
}

