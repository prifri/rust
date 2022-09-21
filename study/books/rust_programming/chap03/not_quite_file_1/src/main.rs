/*
 * prifri, 2022.09.22:
 * - compiler warnning 완화
 */
#![allow(unused_variables)]

/*
 * prifri, 2022.09.22:
 * - typdef File String;
 */
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}


/*
 * prifri, 2022.09.22:
 * - 사용하지 않은 함수에 대해 컴파일러 경고 완화.
 */
#[allow(dead_code)]

/*
 * prifri, 2022.09.22:
 * - !
 * 반환값이 절대로 없다는것을 컴파일러한테 알린다고 설명하는데.
 * void와 같은 건 ()반환 타입을 의미하며, 이건 진짜 이 함수에서 평생 뒤로
 * 돌아갈 일이 없다는것을 의미한다.
 *
 * - ()
 * 길이가 0인 튜플. 함수가 아무 값도 반환하지 않음을 표현하는데 이용된다.
 * 반환 타입이 없는 함수나 세미콜론으로 끝나는 표현식은 ()을 반환한다.
 */

fn read(f: &mut File,
        save_to: &mut Vec<u8>) -> ! {

/*
 * prifri, 2022.09.22:
 * - 프로그램이 이 지점으로 오면 중단된다.
 */
    unimplemented!()
}

fn main() {

/*
 * prifri, 2022.09.22:
 * - String의 모든 method를 상속한다는Emtdlfkrh gksek.
 */
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    //read(f1, vec![]);
    close(&mut f1);
}
