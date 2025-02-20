use super::{Ballot, NodeId};
use either::Either;

/// `PREPARE` message is the Phase 1a message from a proposer sent
/// to acceptors to receive agreement to not accept ballots of lower value.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Prepare(pub Ballot);

/// `ACCEPT` message is the Phase 2a message from a proposer sent
/// to acceptors to accept a value. The `ACCEPT` message is predicated
/// on the proposer receiving quorum from Phase 1.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Accept<V>(pub Ballot, pub V);

/// Either of the proposer message values.
pub type ProposerMsg<V> = Either<Prepare, Accept<V>>;

/// `PROMISE` is the Phase 1b message sent from acceptors in reply to
/// `PREPARE` messages. The ballot in the promise denotes that the acceptor
/// will not accept ballots less than the promised ballot.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Promise<V>(pub Ballot, pub Option<(Ballot, V)>);

impl<V> Promise<V> {
    /// Creates a reply for the `PROMISE`. Promises are returned to the
    /// proposer that originally sent the `PREPARE`.
    pub(crate) fn reply_to(self, node: NodeId) -> Reply<Promise<V>> {
        Reply {
            reply_to: node,
            message: self,
        }
    }
}

/// `REJECT` is sent from an acceptor in reply to a proposer
/// when a ballot is being proposed in a `PREPARE` message or seen in an
/// `ACCEPT` message that preceeds the last promised value from the acceptor.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Reject(pub Ballot, pub Ballot);

impl Reject {
    /// Creates a reply for a `PROMISE` or `ACCEPT` to the proposer
    /// that originated a message with a ballot that preceeded the last
    /// promised.
    pub(crate) fn reply_to(self, node: NodeId) -> Reply<Reject> {
        Reply {
            reply_to: node,
            message: self,
        }
    }
}

/// `ACCEPTED` is the Phase 2b message that is broadcast from acceptors
/// denoting acceptance of a value.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Accepted<V>(pub Ballot, pub V);

/// `RESOLUTION` is the result of a quorum of `ACCEPTED` messages being received.
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Resolution<V>(pub Ballot, pub V);

/// Struct containing the node and message for a single destination.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Reply<M> {
    /// Node identifier that should be sent the reply
    pub reply_to: NodeId,
    /// The reply message
    pub message: M,
}
