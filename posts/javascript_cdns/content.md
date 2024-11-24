
# CDNs

While I was perusing through the building of my blog, I ran into a problem where I did not have syntax
highlighting of code for free. This led me on the journey of figuring out how the general internet 
has been setting up syntax highlighting for blogs. In order to do this, I set out to the ol' reliable 
ChatGPT as a better search engine with some added context.
<br>
<br>

The recommendation I was given from ChatGPT was to use [prismjs](https://prismjs.com). So naturally, I 
end up installing the package itself with NPM and try to build everything myself to serve statically. 
However, this is where I ran into troubles. From not knowing what all needed to be written up so that
I would be able to support every language I wanted syntax highlighting for, to not knowing that the 
css for the syntax highlighting is created independently. This is where I got back to ChatGPT who 
then recommended using a CDN. Particularly, `https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css`.
<br>
<br>

Somewhat naturally to me, seeing a cloudflare link without any clear vetting is something that concerned me.
I hesitated using this and kept on chugging along to try to build everything myself, where I know I am building
something from a venerable source and can trust it. 
<br>
<br>

After struggling to figure this out for a long time, I ended up caving in and using the CDN *just* to see if
it would work... And what do you know, it worked! I could now get syntax highlighting:
```js
const greet = (name) => `Hello, ${name}!`;
```


This was great! But I still wanted to not rely on some 3rd party CDN to be available at all times. So 
what did I do next? Well, I decided to download the css and associated js required for the highlighting
and include it in my `/static` directory. At this point, everything seemed wonderful. I was getting
syntax highlighting, I was able to serve this all statically without depending on some third party
to be available 24/7/365; I was happy.
<br>
<br>

This all came to a tumble when I ended up trying to write some code in python, one of
my more comfortable languages which I use frequently and would frequently have cause to 
write about. Bam! no highlighting. From this point, I realized that the CDN I had pulled from 
only had a few languages it supported. So I went back to [prismjs](https://prismjs.com) and did my 
due diligence before I realized that I could just download the minified css and js for syntax 
highlighting in all the languages that I wanted.
<br>
<br>

Of course, at this point I started coming to the realization on just how much static code 
is served on the daily through CDNs as a means to an easy and dynamic way of injecting 
code. Somehow, psychologically it was more terrifying to me to see minified code that 
I download from less venerable sources (read - not from a package manager or std lib),
than it was to download executables from sources like package managers. Noting here that 
with executables, what is running is much more obfuscated than having the original 
code that you get from CDNs for code from, for example, prismjs. It was truly at this 
moment where I started to realize just how much I take for granted as safe on the 
internet, and moreover my systems. While I have always understood the notorious [xkcd](https://xkcd.com/2347/)
that all modern infrastructure stands on the shoulders of giants, albeit the fragility of the situation
was not something that I truly understood. While I understand at a high level the reality of this
comic, I do not have as deep of an apprecation for it as I will as I further venture into my 
career.
<br>
<br>

This is all just to say, due to the very nature of the internet and the sheer 
depth of dependencies that all modern infrastructure has, that the scale of 
CDNs statically providing code will only increase. Additionally, because of 
the growing globalization of our communications, CDNs will continue to be more 
and more important for latency reasons. Truly it astounds me the degree in which 
we are globalizing our communications networks and spreading information across
the planet.
