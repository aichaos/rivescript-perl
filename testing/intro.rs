> begin
	+ request
	* <get name> == undefined => {topic=newuser}{ok}
	- {ok}
< begin

> topic newuser
	+ *
	- Hello! My name is Soandso! What's your name?

	+ *
	% * what is your name
	- <set name=<formal>>Nice to meet you, <get name>!{topic=random}
< topic

+ *
- Testing...

! sub what's = what is
