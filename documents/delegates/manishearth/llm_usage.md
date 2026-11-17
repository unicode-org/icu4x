# Thoughts on LLM usage in ICU4X

LLMs are turning out to be pretty powerful and useful tools, and they are only getting better. I have certainly found them a major boost to my productivity on ICU4X and in general. I have also noticed various growing pains, as we figure out the best way to use these tools.

I find it useful to look at code and "comments" as different categories. "code" includes markdown files and comments that go in the repo.

## Thoughts on LLM *code* contributions

Overall I find modern LLMs to produce decent code when driven by someone with familiarity with the codebase. It takes some time for people to get used to LLMs, and people still make mistakes.

I find that reviewing LLM code has a *different* feeling than reviewing human code, even when the code has been carefully reviewed and maybe even tweaked by the human later. When a team member writes code by hand, I know that each line of code was a choice by them, conscious or subconscious. To some extent, I trust those minor choices. If a team member uses an LLM to write the code, even if they reviewed it, I find myself reviewing the PR differently.

I've certainly noticed this in code an LLM has produced for me, even after my own review, reviewers might point out minor things where I would not have made that choice were I manually writing the code, but I missed it during my own review because the choice "made sense" when doing a review. A lot of choices while coding are between options that are mostly the same but may have minor salient differences. Some of those minor differences are more major based on a style guide, preferences, or nuanced concerns.

This is not a problem: this is *fine*, "different" doesn't necessarily mean it's much more work, and as either the models have gotten better or as the team has gotten more used to LLMs, I'm finding these differences to be less and less important.

I mostly highlight this to highlight that it is useful to know if something is LLM-prompted, even when it comes from a trusted collaborator. So disclosure is always important.


When it comes to external contributions, however, I find a set of different dynamics. Historically, in the pre-LLM world, if an external contributor wished to contribute, usually they would pick what appears to be a reasonably sized issue. Usually, while working on the code, if they realize that they don't understand something, they would ask questions. The process of actually working on a thing usually helps either cement or shake your confidence in your understanding of the code, and this process is useful.

LLMs tend to be good at laundering confidence. This varies from model to model; some models are pretty good at noticing fishy things and surfacing them, others aren't. When they don't surface them that is of course a problem, but it's often a problem even when they do surface them: it feels very easy to just accept the recommended suggestion from the LLM. This leads to a confident LLM and a confident contributor: the LLM "feels" confident because its driver reassured it to go on, the contributor feels confident because they followed the LLM's recommended suggestion.

As such, I often find that *large* contributor PRs tend to often go down the wrong path completely in ways that did not previously happen.

Overall, I am willing to review LLM contributions from newcomers, but for larger changes or changes to nuanced things I often wish to see engagement from the contributor in ways that shows they are attempting to understand the thing.

I will tend to *deprioritize* LLM PRs from people I don't know. I am very happy to mentor new contributors, but less excited to teach someone else's LLM. I would rather we invest in [documentation by-agents for-agents](https://github.com/unicode-org/icu4x/issues/8556).


## Thoughts on LLM *comments*

Compared to code, I think I am more displeased with the dynamics around LLM commenting. Again, there is a difference between this for trusted collaborators vs other people.

Generally, I *really* don't like the feeling of "talking to an LLM" in toplevel comments. I have my own LLM and can prompt it if I wish. I do not find it useful when someone runs an LLM and it participates in conversation on their behalf. It feels disrespectful. When I am writing, I am expecting the human on the other end to read it.

A sub-dynamic of this I particularly dislike is I don't like having to *disagree* with someone else's LLM. The "I think B is the one that ends this" comment from an LLM on [#8533](https://github.com/unicode-org/icu4x/issues/8533#issuecomment-6052251560) is an example of this: the LLM was confidently wrong and was posing as the person. The LLM simply did not know our constraints, but the comment is written with the confidence of someone who understands what they are talking about. This is unpleasant to interact with. Disclosure would have helped here, but ultimately I do not want to be "talking to an LLM".

I don't have a problem with LLM research. I have a slight preference it be collapsed in a `<details>` because LLMs tend towards verbosity, but this is not a strong one. Note that this is distinct from "talking to an LLM": someone writing a comment as a human and then saying "here's what the LLM had to say" is still me talking to the person, and it does not feel unpleasant to have to contradict the LLMs thoughts. If anything, it feels more collaborative: it feels like the human LLM operator and I are collaborating on getting all the right inputs to the LLM.

When it comes to review comments, I am also fine with clearly disclosed LLM review comments, and I am mostly fine with authors using LLMs to respond to review comments. After all, the LLM wrote the code; it has better answers for many of these questions. I have not been doing this at all on ICU4X (Nov 2027), but I have been doing this on Chromium (where it is more common for people to use LLMs to post review comments), and usually what I do is ask my LLM to go through the comments, fix nits, and then tell me what it thinks about the other comments. After thinking them through, I usually end up going with its suggestions, which also involves the LLM posting responses explaining its choices

I again would prefer not having to be disagreeing with an LLM: if there is disagreement I would like it to be bubbled up to the human after the first back and forth. When I have been using LLMs to respond to review comments I have been making sure that there is no serious disagreement. Most of it has been "yeah that makes sense" acks, "how about this design instead?" discussion-starters, or "here's why I chose to do it this way" intent clarifications.

I also don't have a problem with issue and PR bodies being LLM-generated. While they are in a sense "comments" they are also an artifact like code or documentation. When I comment on an issue I are participating in a discussion but it does not feel like I am "talking to an LLM" just because the issue is LLM-generated

## My ideal LLM policy


For code:
 - LLMs may be used but must be disclosed
 - The author must stand behind the LLM code (or explicitly acknowledge they do not, see [What are we comfortable fully vibecoding](https://github.com/unicode-org/icu4x/issues/8555))
 - Maintainers may choose to not review large LLM PRs from less established contributors
 
For comments:
 - Participate in comment threads as a human first: do not give people the impression they are talking to an LLM. Actually say your thoughts directly before pasting LLM text
 - Clearly-marked LLM research is fine
 - LLM use in review comments and review discussions is fine if disclosed.
 - It is ok for issue and PR bodies to be fully LLM-generated. I do not actually care about disclosure on this.

## Things I strongly want for ICU4X's LLM policy

These are things I would probably push hard for

 - Everything under the "for code" section above
 - LLM comments (discussion comments and review comments) must be disclosed. This does not need to be disclosed once-per comment. LLM use for issue and PR bodies need not be disclosed but it is encouraged.
 - Maintainers are allowed to ask that someone engage in an individual comment thread as a human if they are not feeling respected.
