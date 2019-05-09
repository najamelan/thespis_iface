use crate::{ * } ;


/// Type responsible for knowing how call and send messages to an actor based on an Any pointer
/// ot that actors recipient, and a ServiceID.
///
/// This is the part of the code that is necessarily in the client code, usually by using a macro,
/// because the types of services are not known to the actor implementation.
///
//
pub trait ServiceMap<MS: MultiService, RecipientError>
{
	/// Return a boxed ServiceMap.
	/// This allows for cleaner api's as you don't have to pass a Type parameter and a boxed value.
	//
	fn boxed() -> BoxServiceMap<MS, RecipientError> where Self: Sized;

	/// Send a message to a handler. This should take care of deserialization.
	//
	fn send_service( &self, msg: MS, receiver: &BoxAny );

	/// Call a Service.
	/// This should take care of deserialization. The return address is the address of the peer
	/// to which the serialized answer shall be send.
	//
	fn call_service
	(
		&self                                           ,
		 msg        :  MS                               ,
		 receiver   : &BoxAny                           ,
		 return_addr:  BoxRecipient<MS, RecipientError> ,
	);
}
