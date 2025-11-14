# pyrite
A work-stealing, decentralized task executing network!

## How does it work
At its core, the pyrite network is a basic peer-to-peer network. Each node connects to a network, and discovery is fairly simple. Task spreaders are nodes that do not contribute computation power, but instead contribute tasks to the network. They are responsible for producing work for the network, but they still act like a node themselves, just not one that's willing to do work!
Tasks themselves are sent over as WASM binaries. This means that nodes MUST be able to run WASM, but it ensures that we have platform-independent execution across the network.

### Discovery
Node discovery within the network relies on new nodes knowing at least 1 existing node. Once a node connects, it will tell the node it already knows about its own existence. That node will then spread this info throughout the network, informing each node of the newly connected node.
**/!\ THE PLAN IS TO UPDATE THIS, AS THIS REQUIRES THE TASK SPREADER TO KNOW NODES AS WELL, WHICH KINDA DEFEATS THE POINT REGARDING DECENTRALIZED COMPUTING**

### Task flow
1. Task spreader sends request out to the network, looking for worker nodes
2. Worker nodes, if looking for work, reach out to the task spreader, asking for work.
3. Task spreader picks nodes to send work to based on various factors, like latency and node-performance-score

### Task design
Tasks are built up out of 3 components.
1. A header containing the size of the WASM binary and input data
2. Input data as a raw byte array. Deserialization is handled by the task itself, so the format does not matter!
3. The actual workload as a WASM binary

The result looks as follows:
1. A status code, so we know if a task was succesfully executed or not. This does not mean a succesful execution has resulted in the right output, that is up to the task itself!
2. The length of the resulting byte array
3. The actual task result as a byte array

### WASM environment/limitations
File io is difficult to support. Perhaps I could do file io by only giving the runtime access to a specific directory. Networking should be supported, but it IS a bit scary to give the network the ability to do networking stuff.
