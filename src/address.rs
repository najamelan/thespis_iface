use crate :: { *, import::* };


/// Behavior representing the capability of delivering a message to an actor's mailbox.
///
/// The send method comes from Sink:
///
/// Send a message without wanting a return from the actor. This is a one-way operation.
/// This still returns a future because the mailbox might be async, so delivering the
/// message might be async, but this will resolve as soon as the message is sent to the mailbox.
/// You will not get notified when the message is handled by receiver.
///
/// This returns result because sending to the mailbox might be a fallible action.
/// If any errors happen after the message is sent to the mailbox, you shall not be notified.
/// There shall be no acknowledgement of reception.
//
pub trait Address<M>: AsAddress<M> + Identify

	where  Self: Sink<M> + Any + fmt::Debug + Unpin + Send                                  ,
			 <Self as Sink<M>>::Error: std::error::Error + Send + Sync + fmt::Debug + 'static ,
	       M   : Message                                                                    ,

{
	/// Call an actor and receive the result of the call. This is a two-way operation. Calling with
	/// a message type that has `Return=()` will notify you that the message has been handled by the
	/// receiver.
	//
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call( &mut self, msg: M ) -> Return<'_, Result< <M as Message>::Return, <Self as Sink<M> >::Error >>;

	/// Get a clone of this address as a `Box<Address<M>>`.
	//
	fn clone_box( &self ) -> BoxAddress<M, <Self as Sink<M> >::Error>;
}



/// Allows upcasting to Address if you have a `&dyn Trait` to a trait that extends it. Eg. where `trait Trait: Address<M>`.
/// TODO: usefulness analysis... currently used nowhere.
//
pub trait AsAddress<M>

where  Self: Sink<M> + Any + fmt::Debug + Unpin + Send                        ,
		 <Self as Sink<M>>::Error: std::error::Error + Send + Sync + fmt::Debug ,
       M   : Message                                                          ,

{
	/// Upcast `&self` to `&dyn Address`.
	//
	fn as_address( &self ) -> &dyn Address<M, Error = <Self as Sink<M> >::Error>;
	// fn as_box_address( self:  ) -> Box<dyn Address<M, Error = <Self as Sink<M> >::Error>>;
}


impl<T, M> AsAddress<M> for T

where  T: Address<M> + Sink<M> + Any + fmt::Debug + Unpin + Send           ,
       M: Message                                                          ,
		 <T as Sink<M>>::Error: std::error::Error + Send + Sync + fmt::Debug ,

{
	fn as_address( &self ) -> &dyn Address<M, Error = <Self as Sink<M> >::Error> { self }
}



impl<M, T> Address<M> for Box<T>

	where  M: Message    ,
	       T: Address<M> + Identify,
	       T: Sink<M> + Any + fmt::Debug + Unpin + Send                        ,
	      <T as Sink<M>>::Error: std::error::Error + Send + Sync + fmt::Debug ,
{
	#[ must_use = "Futures do nothing unless polled" ]
	//
	fn call( &mut self, msg: M ) -> Return<'_, Result< <M as Message>::Return, <T as Sink<M> >::Error >>
	{
		(**self).call( msg )
	}

	/// Get a clone of this address as a `Box<Address<M>>`.
	//
	fn clone_box( &self ) -> BoxAddress<M, <T as Sink<M> >::Error>
	{
		(**self).clone_box()
	}
}
