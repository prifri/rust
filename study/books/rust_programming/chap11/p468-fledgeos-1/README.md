책은 old버전이라 현재 상황에서 맞지 않는 듯 싶다. 공식 홈페이지에서 좀 바뀌었던데 해당 내용들로 해도 안된다.

다음을 참고 하였다.

[참고1](https://github.com/rust-osdev/bootimage)
[참고2](https://os.phil-opp.com/minimal-rust-kernel/)
[참고3. rust in action 공식 홈페이지](https://livebook.manning.com/book/rust-in-action/chapter-11/v-14/72)
[core error](https://github.com/phil-opp/blog_os/discussions/998)

# error

1. 책 내용대로하면 다양한 에러가 나온다. 가상머신, llvm 관련이라 그 사이 뭔가 바뀌어서 해당 버전들로 하면 깨진거 같다.

2. 그래서 공식 홈페이지에 들어가면, 역시 안된다. 

3. 그래서 다른 github를 참고해서 보면, 뭔가 진전이 있는데 역시 core::* 어찌고 error 가 나온다.

4. 이런 저런 조합을 쓰다가 build따로, 실행따로 하는법을 찾았다.

5. build.sh -> run.sh을 따로 실행해서 하면 컴파일도 되고 실행도된다.
