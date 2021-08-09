- Finish making the expansion primitive static vs dynamic
- 
- Add an `is_empty` method to the stream trait
- Move the expansion logic to the input driver to handle multiple files
  - Make a StackStream<T> type that we can use (1) for expansion and (2) for multiple input files
- Document what's written.

- Write the input module > reads multiple files at a time, contains the cat code map
    and can be a stream.
  
- Write the `texide tokenize` and `texide expand` command tools.
  - We can make the expand command smart vis-a-vis new lines


- Transcribe the Go implementation of macros to here.
