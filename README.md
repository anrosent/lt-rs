lt-rs
=======

This is an implementation of a Luby Transform code in Rust, more or less a straight port from my Python project.

See _D.J.C, MacKay, 'Information theory, inference, and learning algorithms. Cambridge University Press, 2003_ for reference on the algorithms.

## Encoding

The encoding algorithm follows the given spec, so no innovations there. A few optimizations are made however. First, the CDF of the degree distribution, M(d), is precomputed for all degrees d = 1, ..., K. This CDF is represented as an array mapping index d => M(d), so sampling from the degree distribution mu(d) becomes a linear search through the CDF array looking for the bucket our random number on \[0, 1) landed in. This random number is generated as specified using the linear congruential generator. 

Second, the integer representation of all blocks is held in RAM for maximum speed in block sample generation. This is a limitation on the size of the file practically encoded on most computers, but this decision does not reach far into other parts of the design, and it can be easily addressed if necessary for better memory scalability.

```rust
// TODO:
```

## Decoding
    
The decoder reads the header, then the body, of each incoming block and conducts all possible steps in the belief propagation algorithm on a representation of the source node/check node graph that become possible given the new check node. This is done using an online algorithm, which computes the appropriate messages incrementally and passes them eagerly as the value of source nodes is resolved. Thus, the decoder will finish decoding once it has read only as many blocks is necessary in the stream to decode the file, and it seems to scale well as the file size, and block size increase.

```rust
// TODO:
```

## Progress:

- Sampler completed
- TODO: encoder/decoder APIs
- TODO: executables
