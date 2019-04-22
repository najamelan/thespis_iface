# thespis_iface
The interface of thespis (contains only traits)


## TODO:

- Get rid of the threadsafe variants of address
- oblige all implementors of all traits to implement Debug?
- go over actix features and see what would be useful to have (backpressure?)
- remote Addr? if the actor is known?
- defaults for associated types, like () for Message::Result, and possibility to derive with defaults
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


