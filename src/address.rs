use crate :: { import::*, * };


pub trait Address< A: Actor > : Clone
{
	fn send<M>( &mut self, msg: M ) -> TupleResponse

		where A: Handler< M >                     ,
		      M: Message< Result = () > + 'static ,
	;


	fn call<M: Message + 'static>( &mut self, msg: M ) -> Response<M>

		where A: Handler< M >,
	;


	fn recipient<M>( &self ) -> Box< dyn Recipient<M> >

		where M: Message    + 'static,
		      A: Handler<M> + 'static,
	;
}


pub trait ThreadSafeAddress< A: Actor > : Clone
{
	fn send<M>( &mut self, msg: M ) -> ThreadSafeTupleResponse

	where  A                    : Handler<M>                              ,
	       M                    : Message< Result = () > + Send + 'static ,
	      <M as Message>::Result: Send                                    ,

	;

	fn call<M: Message + 'static>( &mut self, msg: M ) -> ThreadSafeResponse<M>

	where  A                    : Handler<M>     ,
	       M                    : Message + Send ,
	      <M as Message>::Result: Send           ,

	;

	fn recipient<M>( &self ) -> Box< dyn ThreadSafeRecipient<M> >

		where  A                    : Handler<M>     + 'static ,
		       M                    : Message + Send + 'static ,
		      <M as Message>::Result: Send                     ,
	;
}

