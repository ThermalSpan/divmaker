# divmaker

### What and Why
I use this tool to modify the html that pandoc spits out. Now, I'm sure there is a better solution somewhere. Maybe I could configure pandoc better. Maybe I'm missing some html / css knowledge. On the other hand, I can be irrational and I got hooked on this idea.

Specifically, pandoc puts math display blocks in a paragraph, and I want them in a div. The reason being that I can center the text. So where when a paragraph contains a `class="math display"` span, I replace the p with a div, and add the `class="math display"` attribute.

### How

I am using the [html5ever](https://github.com/servo/html5ever) library to parse, manipulate, and serialize html.

The documentation, as of 0.5.4, for html5ever is less than ideal. I found this [stack overflow post](https://stackoverflow.com/questions/38859811/how-do-i-parse-a-page-with-html5ever-modify-the-dom-and-serialize-it) to be a great entry point. I also found the source of [scraper](https://github.com/programble/scraper) to useful in understanding how to interact with html5ever.

### Examples

```
russell$ cat test.html
<p>
	<span class="math display">
        \begin{aligned}
            x &= y^2 & (1)\\
            &= z^2 + 2 & (2)
        \end{aligned}
    </span>
</p>

russell$ divmaker --input test.html

russell$ cat test.html
<div class="display">
    <span class="math display">
        \begin{aligned}
            x &amp;= y^2 &amp; (1)\\
            &amp;= z^2 + 2 &amp; (2)
        \end{aligned}
    </span>
</div>
```

### Future Work

If I find some more use cases for this tool, I might put in the time to generalize it a bit more. For example, using a grammer / config file to describe the transformations.
