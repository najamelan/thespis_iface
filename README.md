# thespis_iface
The interface of thespis (contains only traits)

## References:

- [Actor Model of Computation: Scalable Robust Information Systems](https://arxiv.org/abs/1008.1459) by Carl Hewitt.
- video: [Hewitt, Meijer and Szyperski: The Actor Model (everything you wanted to know...)](https://youtu.be/7erJ1DV_Tlo)

## TODO:

- fix clone on recipient like so: https://stackoverflow.com/revisions/30353928/3
- oblige all implementors of all traits to implement Debug?
- go over actix features and see what would be useful to have (backpressure?)
- remote Addr? if the actor is known compile time?
- defaults for associated types, like () for Message::Return, and possibility to derive with defaults
- how can an actor stop itself, should mb be fed to handle, should there be a method self.mb, should there be a stop method on mailbox
- do some proper research on tokio reactor. Just figured out we don't need a tokio runtime to use stuff that uses epoll. A futures 0.3 executor will do just fine, just using compat on the futures and streams from tokio.
- Really think about meaningful drop as shutdown. Do we want to create a generic method for shutting down actors.
  If it is by sending the actor a message telling it to shut down, be careful these kind of meta messages cannot come
  in from over the network, especially if we ever expose remote actor adresses instead of services.

  Think of a scenario of 2 peers relaying to eachother. The will have an address to eachother, so they will never be
  dropped. Now, if connection A closes, that actor will go in shutdown mode, but peer B will still have it's address to
  relay things. So it won't get dropped. Subscribable events for say something like connection loss would solve this.
  In that case the other peer would be notified, and would drop it's address allowing the mailbox of A to shut down.

  Should an actor have a way to shutdown it's mailbox? One way to make it technically possible it to have the mailbox
  intercept messages. If it gets a message of type MailboxShutdown, it will shut down after the actor has processed
  it's shutdown. The actor won't be able to cancel that though.

- impl recipient for Addr and rename Rcpnt to Receiver?
- client code for remote actors is not generic, it will only work on MultiServiceImpl
- let ServiceMap have a debug implementation which will print out all services with their unique id, so it can be put
  in the documentation of programs and libraries. Peer should probably also be able to tell the remote which services
  it provides.
- clean up benches and write benchmarks for remote actors
- stream handler
- on generic impls, tag methods as default, so that users can override them for specific types.
- remote should store and resend messages for call if we don't get an acknowledgement? If ever you receive twice, you should drop it? Does tcp not guarantee arrival here? What with connection loss? The concept is best efforts to deliver a message at most once.
- Do not return failure::Error from interface, but a library specific error? Then again, this allows impls to define what errors they can throw?
- check mut requirements. we require mut in alot of places, like when sending on an address the address has to be mut. Should we relieve certain of those. It means for example that a struct which holds an addr must also be mut or put it in Refcell just to send messages.
- think about and write up the difference in Send + reply_to and Call. Performance, possiblity to pass on reply_to, link the outgoing send to the incoming response? Possibility to send a reply_to address that is not your own?
- enable CI testing
- enable dependabot
- code coverage?
- impl Sink for references? &'a Addr<A>
- currently spawned tasks swallow panics... We should do something about that.
- try to relax pinning requirements where we can and impl unpin for things like addr to avoid
  forwarding pinning requirements to Actor.
- use juliex and crossbeam channels? in pharos as well?
- can we move the service recipient out of the macro? Like have a generic method on peer that will return
  you a recipient if the peer can reach that service?
- compile time verification that a relay actually relays the sids we are given in a peer?
- use generic-channel crate? allow ring-channel
- we could make a more ergonomic api on peer if we store handlers in service map. It would move code from peer to the macro, but it would allow something like `let (peer_addr, peer_events) = listen_tcp( "127.0.0.1:4343", vec![ service_map1, service_map2 ] );` which is currently impossible, because you need to call register_service on peer with hardcoded types. Service map won't be a zero sized type anymore, but we wouldn't have to keep a bunch of them like we do right now. We could store them consistently like relays, sid -> id, id -> service map. We could even maybe create greater consistency by creating a relay map. It would move responsibility into those objects, lightening the impl of peer.

## Design issues:

- The wire format is a hand baked solution just to get it working. Now we should find out what the final formats might look like. Cap'n proto? or SBE? : https://polysync.io/blog/session-types-for-hearty-codecs

- notion of "one message at a time". This is only necessary if the actor has mutable state. If the actor works without mutable state, it can be run in parallel. In thespis there is currently no way to create this optimization. In actix this is represented by sync actors, where you spawn several of them on different threads and then have one address for the lot. It would be nice if we could have like a mailbox type which shall give you only an immutable reference to self, and now it will process messages in parallel. This will however require a different Handler trait, and that's not very elegant.

- Address is capability. Currently we work on "in process" stuff (the programmer compiling is deemed responsible for the outcome), so not much security considerations are in place. And we work on cross process communication where we consider an all public interface. A process provides services, and if it accepts a connection, it specifies which services will respond on that connection, but it's not more fine grained than that ATM. Eg. We have not implemented any authentication/authorization/encryption/signing.
Consider how you make something available to some entity that is authorized only.

- Two big axises are difficult to implement DRY:
  - Send vs !Send messages, see: https://users.rust-lang.org/t/how-do-you-all-make-your-dynamic-code-send-agnostic/27567/5
  - mut vs not mut actor: would allow processing messages in parallel, but needs a different Handler trait, and Handler trait is used throughout as trait bound A: Handler<M>

- our generic story does not work for remote services right now. Both service map and Peer cannot really
  handle other types. We should work on that and test it. The service_map macro has hardoded references to ServiceID, Peer and needs to create Multiservice to send errors back to the remote. The macro also needs to make multisservices



