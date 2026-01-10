# Content Data

Generate text content, identifiers, and lorem ipsum text.

## Identifiers

### uuid

Generates a UUID v4:

```json
{"fake": ["uuid"]}
```

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]}}}'>
</div>

## Lorem Ipsum

### word

Generates a single random word:

```json
{"fake": ["word"]}
```

<div class="playground-widget" data-config='{"schema":{"word":{"fake":["word"]}}}'>
</div>

### words

Generates multiple words (default 5):

```json
{"fake": ["words"]}
{"fake": ["words", count]}
```

<div class="playground-widget" data-config='{"schema":{"defaultWords":{"fake":["words"]},"threeWords":{"fake":["words",3]},"tenWords":{"fake":["words",10]}}}'>
</div>

### sentence

Generates a sentence with configurable word count:

```json
{"fake": ["sentence"]}
{"fake": ["sentence", min_words, max_words]}
```

<div class="playground-widget" data-config='{"schema":{"shortSentence":{"fake":["sentence",3,5]},"defaultSentence":{"fake":["sentence"]},"longSentence":{"fake":["sentence",10,15]}}}'>
</div>

### paragraph

Generates a paragraph with configurable sentence count:

```json
{"fake": ["paragraph"]}
{"fake": ["paragraph", min_sentences, max_sentences]}
```

<div class="playground-widget" data-config='{"schema":{"shortParagraph":{"fake":["paragraph",2,3]},"defaultParagraph":{"fake":["paragraph"]}}}'>
</div>

## Content Generation Examples

### Blog Post

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]},"title":{"fake":["sentence",4,8]},"excerpt":{"fake":["sentence",10,15]},"content":{"fake":["paragraph",3,5]},"author":{"fake":["name"]},"publishedAt":{"fake":["datetime"]}}}'>
</div>

### Product Description

<div class="playground-widget" data-config='{"schema":{"name":{"fake":["words",3]},"description":{"fake":["paragraph",2,4]},"features":[{"fake":["sentence",5,8]},{"fake":["sentence",5,8]},{"fake":["sentence",5,8]}]}}'>
</div>

### Comments

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]},"author":{"name":{"fake":["name"]},"username":{"fake":["username"]}},"text":{"fake":["paragraph",1,3]},"timestamp":{"fake":["datetime"]}}}' data-batch="3">
</div>
