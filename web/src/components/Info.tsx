const ARTICLE = `# On AI and Why I Still Use AI Anyway

I am going to talk about two topics today:

- The use of AI as a "tool"?
- Why I use AI anyway?

## The AI Companies Want Us to Lose Our Skills

When people talk about everything that is wrong about generative AI, they usually discuss the environment and how the generative AI replacing high quality human work with slop. However, I think there is a greater problem out of all of those.

At first, I did not trust LLMs. Five prompts in, and I already found a mistake. However, then, I began experimenting with programming. It didn't work at first, but after some guiding, it did. I made some cool projects. Then I started letting edit my [dot files](https://codeberg.org/thairanaru/dotfiles). I did not know what the LLM was doing. Yet, it was faster than I could have done myself.

Do you ever notice how at the end of each LLM's response, it often suggest, "If you would like, I could rewrite this paragraph for you?" or "Do you want to discuss why this is the way it is?"? If you use ChatGPT, you would be familiar when it says, "Just say the word, and I'll do it." These deceptive suggestions want us to use the LLM even more. Become dependent on them.

The end goal of generative AI isn't to help us or make the world a better place. It's clear enough that AI companies doesn't care about ethics. It's not a tool; It is a service. The end goal of generative AI is to rob from our skills. And to make us incapable without AI.

## "Just Moderate AI Use"

People will say that we use AI in moderation, we can create a healthy balance and protect our own sovereignty. Yes, that would solve the problem. But that's not happening.

Technologies like social media has already took advantage our attention, driving us incapable to control ourselves. Generative AI is no exception. People often joke about how companies shove generative AI in their faces. Yeah, exactly. It's called removing friction.

Me myself, who usually go to google or YouTube to learn, now have defaulted to Generative AI. I meet new people are completely vibe coding with new [agentic environments](https://github.com/Dcouple-Inc/Pane). As a student under the current job market pressures, it is a difficult ask. AI companies know this too. This is why companies are providing students in a lost, from [Claude Builder Clubs giving away free credits](https://duckduckgo.com/?q=claude+build+club+&ia=web) to [Gemini's One-Year Free offer](https://web.archive.org/web/20260130001321/https://gemini.google/students/).

My idealistic self don't want to use AI. I drive into an adventure into the wildness. Discover and really understand how the world works.

But then reality hits that I am still jobless and none of my cold emails returned warm (okay like two 😭) while the people around you landed internships or creating cool startups. What the hell I'm doing, creating a video game because I was bored? Right now, I can't spend an hour to fix a bug for a game I made for fun.

This is why I use AI.

## Ending Notes

I would appreciate any feedback on this article. Through feedback, I hope to take improves in these articles. No generative AI was used to write this article.

Special acknowledgments for following YouTube channels which inspired me for this article: [Angela D. Collier](https://inv.nadeko.net/channel/UCtscFf8VayggrDYjOwDke_Q), [Hank Green](https://inv.nadeko.net/channel/UCOT2iLov0V7Re7ku_3UBtcQ)

If you are into reading and wondering how generative AI can be manipulative, I also suggest reading this article called [The Structure of Intrinsic Motivation](https://www.annualreviews.org/content/journals/10.1146/annurev-orgpsych-012420-091122). It's so awesome when I first read it.`;

function A({ href, children }: { href: string; children: React.ReactNode }) {
  return (
    <a href={href} target="_blank" rel="noreferrer">
      {children}
    </a>
  );
}

function Block({ q, children }: { q: string; children: React.ReactNode }) {
  return (
    <details className="controls info">
      <summary>{q}</summary>
      <div className="info-body">{children}</div>
    </details>
  );
}

export default function Info() {
  return (
    <>
      <Block q="what is thaimeleon web?">
        <p>
          thaimeleon web is the web version of{" "}
          <A href="https://codeberg.org/thairanaru/thaimeleon">thaimeleon</A>.
          thaimeleon turns your images to color schemes!
        </p>
      </Block>

      <Block q="why use thaimeleon web?">
        <p>
          you should use this over pre-existing color scheme generators because
          thaimeleon focuses on making color schemes that are aesthic while
          being acessible, using{" "}
          <A href="https://github.com/Myndex/deltaphistar">perpetual contrast</A>{" "}
          and <A href="https://bottosson.github.io/posts/oklab/">color spaces</A>
          . it is also configurable!
        </p>
      </Block>

      <Block q="this is slow!">
        <p>
          from testing, the largest bottle neck of the program is purely opening
          the image. try using a different image format. if that does not work,{" "}
          <A href="https://github.com/pbun206/thaimeleon_web/issues">
            please send a bug report
          </A>
          . it also could a rust wasm or react too.
        </p>
      </Block>

      <Block q="i wish thaimeleon did X">
        <p>
          <A href="https://github.com/pbun206/thaimeleon_web/issues">
            please send an issue or patch
          </A>
          . furthermore, i suggest manifesting your preferred color through{" "}
          <A href="https://oklch.com/">a oklch picker</A>
        </p>
      </Block>

      <Block q="how does this work?">
        <p>
          <A href="https://codeberg.org/thairanaru/thaimeleon/src/branch/main/how-it-works.md">
            this
          </A>{" "}
          is no longer up to date, but this should give you a general idea how
          it works
        </p>
      </Block>

      <Block q="ai use">
        <p>
          <A href="https://codeberg.org/thairanaru/thaimeleon">
            the original version
          </A>{" "}
          does not use ai.{" "}
          <A href="https://github.com/pbun206/thaimeleon_web">
            the current, web version
          </A>{" "}
          was built on the original code base and is basically vibe coded. these
          blurbs and the default image was not made with ai
        </p>
        <p>
          why i use ai can be seen below. i am planning to post this to a blog
          which I hope to not vibe code:
        </p>
        <div className="controls-body">
          <textarea value={ARTICLE} readOnly rows={20} spellCheck={false} />
        </div>
        <p>
          tldr: i don't afford the time in this moment bc swe is fucked, but i
          also beilieve this software should be more accessible for everyone
        </p>
      </Block>

      <Block q="source code">
        <ul>
          <li>
            <A href="https://github.com/pbun206/thaimeleon_web">web version</A>
          </li>
          <li>
            <A href="https://codeberg.org/thairanaru/thaimeleon">cli version</A>
          </li>
        </ul>
      </Block>

      <Block q="supporting the creator (me!)">
        <p>
          yes, i am an unemployed university student. no, i don't need money at
          the moment. i ask you to donate that money to{" "}
          <A href="https://www.givedirectly.org/">someone who actually needs it</A>
        </p>
        <p>
          however, what i do need a job
        </p>
        <p>
          give the{" "}
          <A href="https://github.com/pbun206/thaimeleon_web">project</A> a star
          so employers hire me and connect with me on{" "}
          <A href="https://www.linkedin.com/in/peter-bun/">linkedin</A>!
        </p>
      </Block>
    </>
  );
}
