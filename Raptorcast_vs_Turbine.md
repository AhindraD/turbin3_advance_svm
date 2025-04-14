 ## I was comparing Solana's Turbine & Monad's RaptorCast and found out:
  - Monad kind of combines Erasure Encoding with Multicasting:
  - Monad uses a variant of Raptor coding for erasure encoding.
  - Monad uses the RaptorCast protocol instead of traditional multicast methods.

In theory, it looks good, but I had two questions (also including answers I got from their documentation):
Questions here:
Reference: Monad RaptorCast Documentation: https://docs.monad.xyz/monad-arch/consensus/raptorcast 

- ## Q1. Limited Global Support: Multicasting depends on routers and ISPs to support its transmission model. How monad is overcoming this mullticasting limitation?

  - Ans: -  According to Monad Documentaion*

  In the context of Monad's RaptorCast protocol, multicasting is implemented in a way that is suitable for decentralized networks.
  RaptorCast uses a specialized multicast message delivery protocol that doesn't rely on traditional Internet multicasting, which indeed requires support from routers and ISPs.
  Instead, RaptorCast operates over UDP and uses a two-level broadcast tree to distribute messages efficiently among validators.
  This approach allows it to work effectively in a decentralized environment by leveraging erasure coding and redundancy, ensuring reliable message delivery even in the presence of network packet loss or faulty nodes.

- ## Q2. Ineffective with High Node Churn Multicast struggles in dynamic environments where validators frequently join and leave. How monad is overcoming this mullticasting limitation?

  - Ans: According to Monad Documentaion*

  The RaptorCast protocol addresses the challenges of dynamic validator participation by using a two-level broadcast tree for message delivery.
  This approach is designed to handle the dynamic nature of validator sets effectively.
  The message originator is the root of the tree.
  A single non-originator node lives at level 1.
  Every other node lives at level 2.

  Diagram: 
![image](https://github.com/user-attachments/assets/797b0e87-ddae-40e2-b821-76ac16465cbf)




## Follow-Up Questions
 - The answer to the first question is unclear to me.
    How exactly does RaptorCast avoid relying on routers and ISPs while still achieving multicast-like efficiency?

 - For the second question, I don't understand how the two-layer block propagation is efficient.
    What specific advantages does this structure provide over other methods?
    How does it effectively handle dynamic validator participation?
