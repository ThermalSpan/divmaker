# divmaker

### What
I use this tool to modify the html that pandoc spits out. For the moment it is extremely special purpose in the transformations that it makes.

### How

I am using the [html5ever](https://github.com/servo/html5ever) library to parse, manipulate, and serialize html. 

The documentation, as of 0.5.4, for html5ever is less than ideal. I found this [stack overflow post](https://stackoverflow.com/questions/38859811/how-do-i-parse-a-page-with-html5ever-modify-the-dom-and-serialize-it) to be a great entry point. I also found the source of [scraper](https://github.com/programble/scraper) to useful in understanding how to interact with html5ever.

### Examples

```
russell$ cat test.html
<p>
	<span class="math display">
		x^2 = z + c
	</span>
</p>

russell$ divmaker --input test.html

russell$ cat test.html
<p>
	<span class="math display">
		x^2 = z + c
	</span>
</p>

```
        
### Future Work

If I find some more use cases for this tool, I might put in the time to generalize it a bit more. For example, using a grammer / config file to describe the transformations. 
