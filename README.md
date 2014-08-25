## How generative testing would have prevented a weird Rust string bug

[![Build Status](https://travis-ci.org/FranklinChen/rust-string-quickchecked.png)](https://travis-ci.org/FranklinChen/rust-string-quickchecked)

Someone did some really good detective work to find the cause of a bug in Rust's standard string library and [wrote it up](http://www.wabbo.org/blog/2014/22aug_on_bananas.html).

I thought it was really cool, but was also upset that this bug made it into the Rust library code in the first place. Simple generative testing would have prevented this regression bug.
