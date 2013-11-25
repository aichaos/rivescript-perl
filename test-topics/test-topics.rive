! version = 2.00

> begin
	+ request
	- {ok}
< begin

+ *
- Outside of any topics: star trigger

+ hello
- Hello there.

+ test override
- Putting you into a topic that overrides "random"{topic=almostrandom}

+ test override with star
- Putting you into a topic that overrides "random" and has a * trigger{topic=overridestar}

+ test inc
- Putting you into the inc topic{topic=inc}

+ test
- You are not in any topic (proof: <get topic>)

+ exit
@ test

> topic almostrandom inherits random
	+ test
	- You are in the "almostrandom" topic (proof: <get topic>)

	+ exit
	- Leaving the "almostrandom" topic{topic=random}
< topic

> topic overridestar inherits random
	+ test
	- You are in the "overridestar" topic (proof: <get topic>)

	+ exit
	- Leaving the "overridestar" topic{topic=random}

	+ *
	- This is the wildcard inside "overridestar"
< topic

> topic inc includes random
	+ test
	- You are in the "inc" topic (proof: <get topic>)
< topic

// test merging includes with inherits

> topic alpha
	+ alpha trigger
	- Alpha response.

	+ abc
	- abc
< topic

> topic beta
	+ beta trigger
	- Beta response.

	+ xyz
	- xyz
< topic

> topic gamma
	+ gamma trigger
	- Gamma response.

	+ aaa
	- aaa
< topic

> topic mytest includes alpha beta inherits gamma
	+ mytest trigger
	- Mytest response.

	+ bbb
	- bbb
< topic
