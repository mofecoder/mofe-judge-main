INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (2, 'release_test_a', 'Adding', 1, 1, 'A', 'bccf655b-a73a-4f55-ae89-cd5142a1a21f', 'Milk', '$A + B$ の値を求めてください。', '- $0 \\leq A, B \\leq 100$
- $A, B$ は整数', '入力は標準入力から以下の形式で与えられる。
```
$A$ $B$
```', '答えを出力せよ。', '2020-08-11 14:34:21.899694', '2020-08-20 04:00:30.550488', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (3, 'test001_b', 'minimum prise', 5, 8, 'B', '200abcdc-16c9-45a7-84a7-7d00f76ea0ed', 'Assam', 'TTさんはゲーム機を買おうとしています。オンライン通販では $A$ 円で売られています。
近所の家電量販店では $B$ 円ですが、TTさんは今日、家電量販店のセールで全商品 $C$ 割引 (割引後の価格の小数点以下は切り捨て) であることを知っています。<font color="red">(22:25 追記)</font>
さて、TTさんがゲーム機を買うのに払う最小の金額はいくらでしょうか?', '- $1 ≤ A, B ≤ 10^9$
- $0 ≤ C ≤ 10$
- $A, B, C$ は整数', '入力は以下の形式で標準入力から与えられる
```
$A　B　C$
```', '最小値を出力してください。', '2020-08-19 12:48:18.268219', '2020-09-21 13:25:36.376348', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (4, 'test001_c', 'HELP!', 5, 9, 'C', 'b2bea554-9d9e-45b2-a20e-31b9b49843bc', 'Ceylon', 'あなたは最近開店した洋菓子店のオーナーです。
ショーケースの中には $N$ 種類のケーキが一列に並んでおり、それぞれの値段は $a_1, a_2, \\cdots, a_N$ です。

数字に強いこだわりを持つお客さんにケーキを買ってもらうため、隣り合う種類の値段の最大公約数を $1$ より大きくしたいと考えました。
そのためにショーケースに新しい種類のケーキを $0$ 個以上追加したいと思います。

追加するケーキの種類の最小個数を答えてください。
もし、そのような方法がなければ  $-1$  を出力してください。

また、この問題はマルチテストケースです。$ T $ 個のテストケースについて答えてください。', '・ $ 2 \\leq N \\leq  2 \\times 10^4 $
・ $ 1 \\leq a_i \\leq 10^ 9 $
・ $ 1 \\leq T \\leq 10 $
・ 入力はすべて整数です。', '入力は標準入力から与えられます。入力の $1$ 行目は以下の通りです。
``` 
$ T $
``` 
\\
そして、続く $ 2 T$ 行が $T$ 個のテストケースを表します。 
これらはそれぞれ以下の形式の行です。
``` 
$ N $
$ a_1 \\ a_2\\  \\dots \\ a_N $
``` ', '各テストケースに対し、答えを標準出力に出力してください。
なお、テストケースごとに改行をしてください。', '2020-08-19 13:22:45.981364', '2020-09-12 15:35:51.080925', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (5, 'test001_e', 'Strong String', 5, 10, 'E', 'fe9186e7-8c21-44ab-a475-af27f6dc86c7', 'Earlgray', '相異なる文字列 $T$ と $U$ について、$T$ が $U$ よりも **弱い**、すなわち $U$ が $T$ よりも **強い** とは、次の条件のいずれかを満たすことであると定義します。

  - $T$ の長さが $U$ の長さよりも真に小さい。
  - $T$ と $U$ の長さが等しく、かつ $T$ が $U$ よりも辞書順で真に小さい。  

　
  
なお、$T$ が $U$ よりも弱いとき、$U$ が $T$ よりも弱いことはあり得ないことが示せます。

　

$N$ 個の相異なる文字列 $S_1,\\ S_2,\\ \\ldots,\\ S_N$ が与えられます。これらを弱いほうから順に並べてください。より正確には、これら $N$ 個の文字列を並べ替えたもの $S''_1,\\ S''_2,\\ \\ldots,\\ S''_N$ のうちで、以下の条件を満たすような唯一のものを求めてください。

- $1$ 以上 $N - 1$ 以下のすべての整数 $i$ について、$S'' _i$ が $S'' _{i+1}$ よりも弱い。', '- $N$ は整数である
- $2 \\leq N \\leq 10^5$
- $S_i$ は英小文字からなる文字列である
- $1 \\leq |S_i| \\leq 10$
- $S_i \\neq S_j \\\\: (i \\neq j)$', '入力は以下の形式で標準入力から与えられる。

```
$N$
$S_1$
$S_2$
$:$
$S_N$
```', '$N$ 行出力せよ。$i$ 行目には、問題文中の $S''_i$ を出力せよ。', '2020-08-19 13:23:28.157811', '2020-09-13 05:46:40.162804', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (6, null, 'ｑ', null, 8, null, '5dc77cbe-6d06-4f1e-80a6-09a8a99e14c1', 'Milk', 'ｑ', 'ｑｑ', 'ｑ', 'ｑ', '2020-08-19 13:37:11.013988', '2020-08-20 04:21:17.228316', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (7, 'test001_a', 'Compare', 5, 11, 'A', 'a6a3127a-aa86-47f8-8591-59eadbcaf0b2', 'Milk', '$A+B<C$ ですか？', '- $0 \\leq A, B \\leq 100$
- $0 \\leq C \\leq 200$
- $A, B, C$ は整数
', '```
$A$ $B$ $C$
```', '$A+B<C$ ならば  `''Yes''` そうでないならば `''No''` を標準出力に出力せよ。', '2020-08-20 04:18:21.673888', '2020-08-30 11:32:50.646887', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (8, 'test001_d', 'Sqrt Simpilfy', 5, 11, 'D', 'e78fb786-631d-4b33-a6cf-aded84ea8985', 'Ceylon', '正整数 $X$ の（正の）平方根を考えます。$A, B$ を正整数として、$\\sqrt X = A \\sqrt B$ と表せるとき、$B$ を最小化する操作を「簡単にする」といいます。

正整数 $N$ が与えられるので、 $\\sqrt N$ を簡単にしてください。', '- $1 \\leq N \\leq 10^7$
- $N$ は整数', '入力は標準入力から以下の形式で与えられる。
```
$N$
```', '$\\sqrt X$ を簡単にした実数が $A \\sqrt B$ となったとき、$A, B$ を標準出力に出力せよ。ただし、$\\sqrt X$ が整数である場合は、$\\sqrt X$ の値を単独で出力せよ。
', '2020-08-20 04:19:52.944149', '2020-09-21 04:53:10.257891', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (9, 'tea002_a', 'Semiprime-like', 6, 1, 'A', '61f1d2d6-29b9-4b92-bd26-355cab51e97c', 'Assam', '(2020/08/24) 移植に伴い、問題文を一部修正しています。

正整数 $N$ が与えられます。$N$ が以下の条件を満たすかどうかを判定してください。

*条件*
- $N = a \\times b$ を満たす $2$ 以上の正整数 $a, b$ が存在する。', '- $1 \\leq N \\leq 100$
- $N$ は整数', '入力は以下の形式で標準入力から与えられる。
```
$N$
```', '条件を満たすなら `''Yes''`、満たさないなら `''No''`と出力してください。', '2020-08-24 09:21:09.997408', '2020-08-24 10:55:11.626294', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (10, 'tea002_b', 'trick of treat', 6, 1, 'B', 'ef29a873-ad90-4af0-ae64-28c93a3b865b', 'Benihuki', 'Ceylon君は飴が入った4つの箱を持っています。
Ceylon君は全ての箱に入ってる飴の数を均等にしたいと思いましたが、箱の中身は見えないようになっています、
また、飴も地面に置かれると爆発するので中身を全部取り出して振り分けるようなことはできません。
そこで、重みから飴の量を比較できることに気づいたCeylon君は、以下の操作を飴の数が均等になるまで繰り返し行うことにしました。
・最も飴の数が多い箱から飴を1つ取り出して最も飴の数が少ない箱に移す
さて、あなたはそれぞれの箱に $A, B, C, D$ 個の箱が入ってることを知っています。
Ceylon君に箱の中にある飴の数を全て同じにできるか教えてください。', '- $1 \\leq A, B, C, D \\leq 10^5$
- 入力はすべて整数', '入力は以下の形式で標準入力から与えられる。
```
$A$ $B$ $C$ $D$
```', '箱の中にある飴の数を全て同じにできるなら `''Yes''`、そうでない場合 `''No''` を出力してください。', '2020-08-24 09:38:53.631053', '2020-08-30 11:07:50.204698', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (11, 'tea002_c', '最小全域木？なにそれおいしいの？', 6, 1, 'C', 'e8024b6d-86be-40e3-bfef-74611fdee3e3', 'Earlgray', '$N$ 個の街を相互に行き来可能とするように街道を作りたいです。
具体的には、各街からいくつかの街と街道を通って、すべての街へ行けるようにしたいです。
今、街を $1,2,...,N$ として、2 つの街 $(i, j)$ 同士をつなぐ街道に対するコスト $C_{i, j}$ が以下のように推定されています。
- $C_{i, j} = i+j\\ (i \\neq j, 1 \\leq i,j \\leq N)$

この時、$N-1$ 個の街道を建設して、$N$ 個の街が相互に行き来可能となるようにするために必要なコストの最小値を求めなさい。

<details>
<summary>余談</summary>
$N$ 個以上の街道を建設すると最小のコストにはならないことは示せます。<br>
このような問題は一般に最小全域木と呼ばれ、AtCoderでは500点以上の問題に出題されることが多いです。
ただし、この問題を解くには最小全域木のアルゴリズムは必要なく、想定解の1つは $О(1)$ です。
もちろん、最小全域木のアルゴリズムを用いても解くことが出来ます。
</details>
<details>
<summary>ヒント</summary>

- $N=3$，$N=5$ の時のサンプルの図を見て、どの街道を選べば最小のコストになるかを考えてみる。
- $N$ と $N+1$ の答えに法則性があるかを考えてみる。
- 最小コストになる街道の繋げ方には決まりがある？

といったことを考えてみましょう。以下のようにしても良いですが、UnionFind等の知識が必要になり、難易度は高いです。
- 最小全域木のアルゴリズムを調べて貼る。(上級者向け)
</details>', '- $3 \\leq N \\leq 10^3$
- $N$ は整数', '入力は以下の形式で標準入力から与えられる。
```
$N$
```', 'コストが最小となるように街道を作った際に必要なコストを出力せよ。', '2020-08-24 09:57:07.328683', '2020-08-24 10:08:45.138510', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (12, 'tea002_d', 'Multiple Multiple', 6, 1, 'D', '8e47d613-4fe7-43dc-bfee-66c357cc4cd4', 'Flavor', '長さ $N$ の数列 $A$ があります。

$A$ の要素から一つ以上選んで総乗(全ての積)をとったとき、値が $M$ に最も近くなるものを求めて下さい。
具体的には、以下の問題を解いてください。
・$1$ 以上 $N$ 以下の相異なる $k\\ (k = 1, 2, \\dots, N)$ 個の整数 $p_1, p_2, \\dots, p_k$ に対して $\\prod_{i=1}^k A_{p_i} = X$ としたとき、$|X-M|$ が最も小さくなる $X$ を求めて下さい。

なお、答えが複数ある場合は値が最も小さいものを出力してください。', '- $1 \\leq N \\leq 18$
- $1 \\leq M \\leq 10^{18}$
- $-10 \\leq A_i \\leq 10$
- 入力はすべて整数', '入力は以下の形式で標準入力から与えられる。
```
$N$ $M$
$A_1\\ A_2\\ \\dots\\ A_N$', '答えを出力してください。', '2020-08-24 10:06:47.836934', '2020-08-24 10:09:31.100005', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (13, 'tea004_a', 'Game Master', 7, 1, 'A', 'd9d545a0-27f4-4ec6-a3b2-f9bd17fc3c0e', 'Milk', 'Mitsubachi君はゲームが大好きです。
このゲームには敵 X と敵 Y が存在し、敵 X を倒すと1体につき $A$ ポイント、敵 Y を倒すと1体につき $B$ ポイントを獲得できます。
Mitsubachi君が敵 X を $C$ 体、敵 Y を $D$ 体倒したとき、Mitsubachi君が獲得したポイントを求めてください。', '- $1 \\leq A, B, C, D \\leq 100$
- 入力はすべて整数', '入力は標準入力から以下の形式で与えられる。
```
$A$ $B$ $C$ $D$
```', 'Mitsubachi君が獲得したポイントを出力してください。', '2020-08-24 13:23:38.100741', '2020-08-24 14:27:39.006427', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (14, 'tea004_b', 'Drill', 7, 1, 'B', '2487ae62-618e-4507-a0aa-a80c389b027f', 'Assam', 'Ceylonさんは冬休みの宿題の算数ドリルを解いています。しかし、Ceylonさんは分数の計算が苦手です。
二つの分数 $\\cfrac{n_1}{d_1}, \\cfrac{n_2}{d_2}$ と演算子 $P$ (`''+''`, `''-''`, `''*''`, `''/''`) が与えられます。
Ceylonさんの代わりに $\\cfrac{n_1}{d_1} P \\cfrac{n_2}{d_2}$ の値を求めてください。
ただし、Ceylonさんの学校の先生は頭がかたいので、きちんと整理された形でないと不正解になります。', '- $1 \\leq n_1, n_2, d_1, d_2 \\leq 10^9$
- 入力はすべて整数', '入力は標準入力から以下の形式で与えられる。
```
$n_1$ $d_1$
$P$
$n_2$ $d_2$
```', '答えとなる既約分数 $\\cfrac{N}{D}$ の $N$ と $D$ $(D>0)$ を空白区切りで出力してください。
ただし、分母 $D=1$ となる場合は整数の形で $N$ だけを出力してください。', '2020-08-24 13:26:54.148129', '2020-08-30 11:07:36.233585', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (15, 'tea004_c', 'Sum on Blackboard', 7, 1, 'C', '0e24a1ad-a5d4-4d16-8709-dd39f5491634', 'Benihuki', '黒板に $N$ 個の整数 $A_1, \\cdots, A_N$ が書かれています。
あなたは次の $Q$ 回の操作を順に行います。
- $i$ 回目の操作では、黒板に整数 $X_i$ が書かれていれば、それらをすべて $Y_i$ に書き換える。

$Q$ 回の操作を終えた後の、黒板に書かれている整数の総和を求めてください。
答えは $32$ bit整数型に収まらない可能性があります。', '- $1 \\leq N \\leq 10^5$
- $0 \\leq Q \\leq 10^5$
- $0 \\leq A_i \\leq 10^5\\ (1 \\leq i \\leq N)$
- $0 \\leq X_i, Y_i \\leq 10^5\\ (1 \\leq i \\leq Q)$
- $X_i \\neq Y_i$
- 入力はすべて整数', '入力は標準入力から以下の形式で与えられる。
```
$N$ $Q$
$A_1\\ A_2\\ \\dots\\ A_N$
$X_1\\ Y_1$
$\\vdots$
$X_Q\\ Y_Q$
```', '$Q$ 回の操作後の、黒板に書かれている整数の総和を出力してください。', '2020-08-24 13:32:22.648893', '2020-08-30 11:08:48.149278', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (16, 'tea004_d', 'Tea Set', 7, 1, 'D', 'b2d73644-beab-4947-8527-b47aef7c6fd2', 'Darjeeling', 'Kaffa くんはティーブレイクを楽しむために紅茶を選んでいます．
今，Kaffa くんの机の上には $N$ 個の紅茶が一列に並んでいます．
また，それぞれの紅茶には種類があり，長さ $N$ の英小文字からなる文字列 $S$ で表されます．$i$ 番目の紅茶の種類は $S$ の $i$ 番目の文字 $S_i$ で表されます．
Kaffa くんはこのなかから連続するいくつかの紅茶を選択するつもりです．また，Kaffa 君は味をたくさん楽しむために，選んだ紅茶の中にちょうど $K$ 種類の紅茶が含まれるように選ぼうと考えています．
そのような選び方の個数を求めて下さい．', '- $1 \\leq N \\leq 2000$
- $1 \\leq K \\leq 26$
- $N, K$ は整数
- $S$ は英小文字からなる $N$ 文字の文字列', '入力は標準入力から以下の形式で与えられる．
```
$N$ $K$
$S$
```', '選び方の個数を整数で一行に出力してください．', '2020-08-24 13:38:51.626873', '2020-08-30 11:09:33.532186', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (17, 'tea004_e', 'Books', 7, 1, 'E', '441c0059-e353-4a51-a719-346be09657ef', 'Earlgray', 'Cafe 中学校には、 $N$ 人の生徒がいます。生徒には、それぞれ $1, 2, \\dots, N$ の番号が振られています。
null 先生は、kichi2004 君が書いたとある本を、生徒全員に読ませたいと思っています。
Cafe 中学校には $M$ の友達関係があり、生徒 $u_i$ と $v_i$ は友達です。友達同士では本を渡すことができます。
kichi2004 君の本はとても高いので、null 先生は、用意する本の数をなるべく少なくしたいと考えています。
用意する本の数を最小化したとき、必要な本の数を求めてください。', '- $1 \\leq N \\leq 10^5$
- $0 \\leq M \\leq \\min(10^5, N(N-1)/2)$
- $1 \\leq u_i, v_i \\leq N$ ($1 \\leq i \\leq M$)
- $u_i \\neq v_i$
- $u_i = u_j$ のとき、$v_i \\neq v_j$
- $u_i = v_j$ のとき、$v_i \\neq u_j$
- 入力はすべて整数', '入力は標準入力から以下の形式で与えられる。
```
$N$ $M$
$u_1\\ v_1$
$\\vdots$
$u_M\\ v_M$
```', '問題文で指定された必要な本の数を、標準出力に出力せよ。', '2020-08-24 13:42:54.172190', '2020-08-30 11:09:55.030068', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (18, 'tea004_f', 'ANTimatter', 7, 1, 'F', 'b75f1f17-c466-4d79-8be1-c4ca3d21ba4d', 'ผักชี', '長さ $10^9$ の線分上にアリが $N$ 匹おり、線分に沿って毎秒 $1$ の距離を歩いています。
アリが線分の端点に到達したとき、そのアリは線分から落下します。

アリはこの線分上を右か左の方向に進みます。そして、左に進んでいるアリと右に進んでいるアリが衝突したとき、両方のアリが瞬時に消滅します。

いま、$N$匹のアリに関する情報として文字列$S$が与えられます。
$S$の $i$ 文字目 $(1 \\leq i \\leq N)$ が `''<''` のとき、線分の左から $i$ 番目にいるアリが左を向いていることを表します。
同様に、$S$の $i$ 文字目が `''>''` のとき、線分の左から $i$ 番目にいるアリが右を向いていることを表します。
また、$S$の $i$ 文字目が `''?''` のとき、そのアリがどちらを向いているかはわかっていません。

アリの向きとしてありえる組み合わせは、$S$ の中にある `''?''` の個数を $T$ として、$2^T$ 通り存在します。
それら全ての組み合わせに対し、十分な時間が経った後線分から落ちたアリの数をそれぞれ足し合わせたものを求めてください。
ただし、答えは非常に大きくなることがあるので $10^9+7$ で割った余りを求めてください。', '- $1 \\leq N \\leq 300$
- $N$ は整数
- $S$ は `''<''`, `''>''`, `''?''` のみからなる $N$ 文字の文字列', '入力は標準入力から以下の形式で与えられる。
```
$N$
$S$
```', '答えを $10^9 + 7$ で割った余りを出力してください。', '2020-08-24 13:46:51.059879', '2020-08-24 14:27:51.194103', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (19, 'test001_f', 'UTBE', 5, 42, 'F', '09d02c14-f425-42f6-9a27-80ebaa2a6e19', 'Flavor', 'チューブの中に $N$ 個の玉が詰まっています。玉にはそれぞれ $1$ から $N$ の番号が割り当てられており、チューブの左端にあるものから順番に $A_1,A_2,\\cdots ,A_N$ とします。ここで、このチューブに対して以下の操作を施していきます。  
  
- チューブ内でもっとも番号の小さい玉が端(左右を問わない)にあるとき、これを石に変える  
- チューブの左端から玉or石を $1$ つ取り出し、右端へ入れる  
- チューブの右端から玉or石を $1$ つ取り出し、左端へ入れる  
  
チューブ内のすべての玉を石に変えるのに必要な操作回数の最小値を求めてください。', '- $1\\leq N \\leq 150000$  
- $1\\leq A_i \\leq N$  
- $A$ は $1$ から $N$ までの整数を並び替えた順列  
- 入力はすべて整数です', '入力は以下の形式で与えられる。  
```
$N$  
$A_1\\quad A_2\\quad \\cdots A_N$
```', 'チューブ内のすべての玉を石に変えるのに必要な操作回数の最小値を出力してください。', '2020-08-27 14:22:24.410162', '2020-09-13 06:05:23.138142', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (20, 'joi_summer_seminner_2020_a', 'フランスパンの切り分け', 8, 93, 'A', '5f46292c-ca77-4bef-8d00-21275ed5d2c8', 'Earlgray', 'J 君・ O 君・ I 君が，JOI 夏期セミナー開催を祝うパーティーに参加した．パーティーでは $1$ つの大きいフランスパンが食事として出される．フランスパンは以下の図の通りに，$N - 1$ 個の切れ目によって $N$ 個の区間に分かれており，左から $i$ 番目の区間の長さは $A_i$ センチメートルである．

![](https://i.imgur.com/Mk1u5CQ.png)

$3$ 人は以下の方法によって，フランスパンを 3 等分したい．
- まず，$N − 1$ 個の切れ目のうち $2$ 個を選び，これらに沿ってパンを切る．
- そうすると，フランスパンは $3$ つの区間に分かれる．一番左をJ 君，真ん中を O 君，一番右を I 君の取り分とする．

以下の図は $N = 5$ の場合の切り分け方の例である．

![](https://i.imgur.com/6JbCLHL.png)

フランスパンの取り分の長さが，$3$ 人ともすべて同じになる切り分け方は存在するか．', '- $1 \\leq T \\leq 10$ ．
- $3 \\leq N \\leq 1500$ ．
- $1 \\leq A_i \\leq 100$ ．
- 入力はすべて整数である．

| 小課題 | 得点 | 内容 | 入力データ番号 | $T$ の値 |
| :----: | :----: | :----: | :----: | :----: |
| 0 | - | サンプルテストケース | #0 | - |
| 1 | 25 | $N = 3$． | #1 | $5$ |
| 2 | 50 | $N \\leq 50$． | #2 | $5$ |
| 3 | 25 | 追加の制約はない． | #3 | $10$ |

', '**各入力データ** について，以下の形式で入力が与えられる．

```
$T$
($1$ 個目のテストケースの情報)
($2$ 個目のテストケースの情報)
$...$
($T$ 個目のテストケースの情報)
```

**各テストケース** について，以下の形式で入力が与えられる．
- $1$ 行目に，整数 $N$ が与えられる．
- $2$ 行目に，整数 $A_1, A_2, A_3, ..., A_n$ が空白区切りで与えられる．', '$T$ 行に渡って出力せよ．
$i$ 行目には，$i$ 個目のテストケースにおける答えを，以下の通りに出力せよ．

* フランスパンの取り分の長さが $3$ 人ともすべて同じになる切り分け方が存在する場合は，`''Yay!''` と出力し，そうでない場合は `'':(''` と出力する．', '2020-08-28 04:39:41.965340', '2020-12-11 10:33:51.671317', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (21, 'joi_summer_seminner_2020_b', 'JOI 悪徳商店', 8, 93, 'B', 'f7bcb373-f1f7-469e-be22-92a3a4e7585f', 'Flavor', 'IOI 王国には，悪質な販売で有名な「JOI 商店」がある．JOI 商店は 1 種類の商品しか売っておらず，毎日値段が変動する．既に今後 $N$ 日間の値段は公開されており，今日から数えて $i$ 日目(以後，$i$ 日目と記す)の商品の値段は $A_i$ 円である．

JOI 商店では，1 日当たり 2 個以上の商品を買うことはできず，すなわち各日について「1個商品を買う」「買わない」という 2 種類の選択のうちいずれかしかできない．また，2 日以上連続で商品を買う場合は，次表の通りに割引あるいは割増される．（連続購入日数は，買わなかった日があればリセットされることに注意せよ．）ただし，10 割引以上の場合は，商品を買うと逆にお金がもらえることを意味する．

| 連続購入日数 | 発生する割引／割増 |
| :----: | :----: |
| 連続購入 1 日目 | 何も起こらない |
| 連続購入 2 日目 | 7 割引（その日の商品が 0.3 倍の価格で買える） |
| 連続購入 3 日目 | 7 割増（その日の商品が 1.7 倍の価格で買える） |
| 連続購入 4 日目 | 14 割引（その日の商品が -0.4 倍の価格で買える） |
| 連続購入 5 日目 | 14 割増（その日の商品が 2.4 倍の価格で買える） |
| 連続購入 6 日目 | 21 割引（その日の商品が -1.1 倍の価格で買える） |
| 連続購入 7 日目 | 21 割増（その日の商品が 3.1 倍の価格で買える） |
| 連続購入 8 日目 | 28 割引（その日の商品が -1.8 倍の価格で買える） |
| 連続購入 9 日目 | 28 割増（その日の商品が 3.8 倍の価格で買える） |
| 連続購入 10 日目 | 35 割引（その日の商品が -2.5 倍の価格で買える） |
|$\\vdots$|$\\vdots$|

E869120 君は JOI 商店に $N$ 日間通い，金儲けに挑戦することにした．彼は最大で何円の利益を得ることができるか．ただし，どの日も商品を買わなかった場合の利益は $0$ 円であり，利益は負になることもある．

![](https://i.imgur.com/WkFvydA.png)', '- $1 \\leq T \\leq 10$
- $1 \\leq N \\leq 50$
- $10 \\leq A_i \\leq 100000$
- **$A_i$ は $10$ の倍数である．**
- 入力はすべて整数である．

この課題は，3 個の小課題から成る．

| 小課題 | 得点 | 内容 | 入力データ番号 | $T$ の値 |
| :----: | :----: | :----: | :----: | :----: |
| 0 | - | サンプルテストケース | #0 | - |
| 1 | 15 | $A_1 = A_2 = A_3 = ... = A_N$| #1 | 5 |
| 2 | 35 | $N ≤ 16$ | #2 | 10 |
| 3 | 50 | 追加の制約はない． | #3 | 10 |', '**各入力データ** について,以下の形式で入力が与えられる.

```
$T$
($1$ 個目のテストケースの情報)
($2$ 個目のテストケースの情報)
...
($T$ 個目のテストケースの情報)
```

**各テストケース** について,以下の形式で入力が与えられる.
- 1 行目に，整数 $N$ が与えられる.
- 2 行目に，整数 $A_1, A_2, A_3, ... , A_N$ が空白区切りで与えられる.', '$T$ 行に渡って出力せよ．
$i$ 行目には，$i$ 個目のテストケースにおける答えを **整数形式** で出力せよ．', '2020-08-28 08:14:13.170513', '2020-12-11 10:34:52.833278', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (22, 'joi_summer_seminner_2020_c', '魔法の宝石 2 (magical Jewels 2)', 8, 94, 'C', '1667e724-17da-4929-a404-031ee5bfb0c8', 'ผักชี', '$H$ 行 $W$ 列のマス目がある．上から $i$ 行目，左から $j$ 列目のマスをマス $(i,j)$ と呼ぶことにする． 各マスには $1$ つの宝石があり，マス $(i,j)$ には種類 $s_{i,j}$ の宝石が落ちている．
ただし， 落ちている宝石の種類は `A`, `B`, `C`, `D`, `E`, `F`, `G`, `H`, `I`, `J` の 10 種類のうちいずれかである．
さて，JOI 君は整数 $a, b, c, d\\ (1 ≤ a ≤ b ≤ H, 1 ≤ c ≤ d ≤ W)$ を選び，$a ≤ x ≤ b,\\ c ≤ y ≤ d$ を満たす全てのマス $(x, y)$ にある宝石を拾う．そのとき，以下の条件を満たせば魔法が発動し，JOI 君は次の国際情報オリンピック(IOI)の日本代表になることができるだろう．

```
「中途半端に拾った」宝石の種類がひとつもない．より具体的には，「1 個以上その種類の宝石を拾ったが，マス目にあるその種類の宝石をすべて拾ってはいない」といった種類の宝石が存在しない．
```

さて，魔法が発動するような整数 $(a, b, c, d)$ の組の選び方は何通りあるか求めよ．答えは大きくなる場合があるので，64 ビット整数を扱うことを勧める．', '- $1 ≤ T ≤ 5$
- $1 ≤ H ≤ 1000$
- $1 ≤ W ≤ 1000$
- $s_{i,j}$ は  `A`, `B`, `C`, `D`, `E`, `F`, `G`, `H`, `I`, `J` のいずれかである．
- $s_{i,j}$ 以外の入力は全て整数である．

この課題は，5 個の小課題から成る．

| 小課題 | 得点 | 内容 | 入力データ番号 | $T$ の値 |
| :----: | :----: | :----: | :----: | :----: |
| 0 | - | サンプルテストケース | #0 | - |
| 1 | 8 | $H = 1,\\ W ≤ 10$| #1 | 5 |
| 2 | 14 | $H ≤ 10, W ≤ 10$ | #2 | 5 |
| 3 | 20 | $H ≤ 55, W ≤ 55$ | #3 | 5 |
| 4 | 21 | $s_{i,j}$ は `A`, `B`, `C` のいずれかである． | #4 | 5 |
| 5 | 37 | 追加の制約はない． | #5 | 5 |', '**各入力データ**について、以下の形式で入力が与えられる。

```
T
(1 個目のテストケースの情報)
```

**各テストケース**について、以下の形式で入力が与えられる．

- 1 行目に，整数 $H,W$ が空白区切りで与えられる．
- $1 + i (1 ≤ i ≤ W)$ 行目に，$s_{i,1}, s_{i,2}, s_{i,3}, ... , s_{i,W}$ が繋がった $W$ 文字の文字列が与えられる.', '$T$ 行に渡って出力せよ．
$i$ 行目には，$i$ 個目のテストケースにおける答えを出力せよ．', '2020-08-28 08:31:01.118521', '2020-08-31 00:21:11.053782', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (23, 'joi_summer_seminner_2020_d', 'よし、グラフだ！', 8, 93, 'D', '0dd84dae-e9de-4954-baa2-aa9537153512', 'ผักชี', 'I 教授と T 教授は，グラフの問題が好きである．
今日は，以下の問題について考えてみることにした．

```
JOI 王国には，$1, 2, ... , N$ と番号付けられた $N$ 個の都市と，$M$ 本の道路がある．$i$ 本目の道路は番号 $A_i$ の都市と番号 $B_i$ の都市を双方向に繋いでおり，文字 $C_i$ が書かれている．

E869120 君は，都市 $1$ から都市 $N$ へ，道路のみを使って移動することにした．道路に書かれている文字を通った順番に全てメモしたとき，JOI 文字列になるように移動することはできるか．もしできる場合は， 作れる最短の JOI 文字列の長さはいくつか．

ただし，同じ都市を二度通ったり，同じ道路を二度通ったりする移動方法も許されるものとする．
```

ただし，**JOI 文字列** とは，以下の条件をすべて満たす文字列のことを指す．JOI 文字列としては，例えば `''JOI''`, `''JJOOII''`, `''JJJOOOIII''` などが挙げられる．
- 文字列の長さ `|S|` が 3 の倍数である．
- 最初の $|S|/3$ 文字が `''J''`，次の $|S|/3$ 文字が `''O''`，最後の $|S|/3$ 文字が `''I''` である．

教授たちに代わって，上枠の問題を解け．', '- $1 \\leq T \\leq 5$
- $2 \\leq N \\leq 20000$
- $1 \\leq M \\leq 20000$
- $1 \\leq A_i < B_i \\leq N$
- $(A_i, B_i) \\neq (A_j, B_j)$
- $C_i$ は `''J''`, `''O''`, `''I''` のいずれかである．
- どの都市からどの都市へも，いくつかの道路を通じて行き来することができる．
- $C_i$ 以外の入力はすべて整数である．

この課題は，7 個の小課題から成る．

| 小課題 | 得点 | 内容 | 入力データ番号 | $T$ の値 |
| :-: | :-: | :-: | :-: | :-: |
| 0 | - | サンプルテストケース | #0 | - |
| 1 | 1 | $N = 2$ | #1 | 3 |
| 2 | 15 | $M = N − 1$ $(A_i, B_i) = (i, i + 1) [1 \\leq i \\leq M]$ | #2 | 5 |
| 3 | 7 | $M = N − 1$．3 本以上の道路と直接繋がっている都市は存在しない． | #3 | 5 |
| 4 | 5 | $M = N − 1$ | #4 | 5 |
| 5 | 25 | $N \\leq 35, M \\leq 35$．JOI 文字列が作れる場合，長さ $36$ 以下の JOI 文字列が作れる． | #5 | 5 |
| 6 | 16 | $N ≤ 700, M ≤ 700$ | #6 | 5 |
| 7 | 31 | 追加の制約はない． | #7 | 5 |', '**各入力データ** について,以下の形式で入力が与えられる．

```
$T$
($1$ 個目のテストケースの情報)
($2$ 個目のテストケースの情報)
...
($T$ 個目のテストケースの情報)
```

**各テストケース** について,以下の形式で入力が与えられる．
- 1 行目に，整数 $N, M$ が空白区切りで与えられる．
- $1 + i (1 \\leq i \\leq N)$ 行目に，整数 $A_i, B_i, C_i$ が空白区切りで与えられる．', '$T$ 行に渡って出力せよ．
$i$ 行目には，$i$ 個目のテストケースにおける答えを，以下の通りに出力せよ．

```
メモした文字列を JOI 文字列にすることができない場合，$-1$ と出力せよ．
メモした文字列を JOI 文字列にすることができる場合，文字列の最短の長さを出力せよ．
```', '2020-08-28 08:39:04.571950', '2020-12-11 10:38:44.569910', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (24, 'joi_summer_seminner_2020_e', 'ミーティング 2.0 (Meetings 2.0)', 8, 94, 'E', '22100ae5-0a1f-4bfc-a031-eb3aaa5c638a', 'ผักชี', '新 JOI 通りには $N$ 個のビルが左から右に一列に並んでいる．左から $i$ 番目のビル (以下，ビル $i$ と記す) の高さは $H_i$ キロメートルであり，隣り合うビルはちょうど $1$ キロメートル離れている．各ビルには $1$ 人の住人が屋上に住んでおり，全員秒速 $1$ キロメートル
で歩くことができる．

![問題イメージの画像](https://i.imgur.com/yAdXjpX.png)

この通りでは，今年は $Q$ 個の会議が開催される予定である．$i$ 個目の会議では，ビル $L_i,\\ L_i+1\\ ,\\ ...\\ ,\\ R_i$ の住民が一つのビル $x\\ (L_i ≤ x ≤ R_i)$ に集まらなければならない．会議をできるだけ迅速に進めるため，「集まる時間」が最短となるようなビル $x$ を選ぶことにした．
「集まる時間」は以下のように定義される．

```
「集まる時間」は会議の参加者のうち最も移動に時間を要する住人の移動時間である．
ただし，ビル $t$ の住人がビル $x$ に移動するときにかかる時間は，次式で表される．

$H_t +|t − x| + H_x$

ここでは，ビル $x$ の住人も移動しなければならないことに注意せよ．
```

各会議における，最短の「集まる時間」をそれぞれ求めよ．


', '- $1 ≤ T ≤ 3$
- $2 ≤ N ≤ 200\\ 000$
- $1 ≤ Q ≤ 200\\ 000$
- $1 ≤ H_i ≤ 200\\ 000$
- $1 ≤ L_i ≤ R_i ≤ N$
- 入力はすべて整数である．

この課題は，8 個の小課題から成る．
ただし，小課題 6 の制約，入力全体についての制約ではなく，各テストケースについての制約であることに注意せよ．

| 小課題 | 得点 | 内容 | 入力データ番号 | $T$ の値 |
| :----: | :----: | :----: | :----: | :----: |
| 0 | - | サンプルテストケース | #0 | - |
| 1 | 9 | $N ≤ 300,\\ Q ≤ 300$ | #1 | 3 |
| 2 | 16 | $N ≤ 2000,\\ Q ≤ 2000$ | #2 | 3 |
| 3 | 6 | $N ≤ 100000,\\ Q ≤ 100000,\\ H_i ≤ 2$ | #3 | 3 |
| 4 | 6 | $N ≤ 100000,\\ Q ≤ 100000,\\ H_i ≤ 6$ | #4 | 3 |
| 5 | 11 | $N ≤ 100000,\\ Q ≤ 100000,\\ H_i ≤ 100$ | #5 | 3 |
| 6 | 13 | $N ≤ 100000,\\ Q ≤ 100000$． $H_i$ には高々 $100$ 種類の値しかない． | #6 | 3 |
| 7 | 34 | $N ≤ 100000,\\ Q ≤ 100000$ | #7 | 3 |
| 8 | 5 | 追加の制約はない． | #8 | 3 |', '**各入力データ** について，以下の形式で入力が与えられる．
```
$T$
(1 個目のテストケースの情報)
(2 個目のテストケースの情報)
...
($T$ 個目のテストケースの情報)
```

**各テストケース**について，以下の形式で入力が与えられる.
- 1 行目に，整数 $N, Q$ が空白区切りで与えられる.
- 2 行目に，整数 $H_1, H_2, H_3, ... , H_N$ が空白区切りで与えられる.
- $2 + i\\ (1 ≤ i ≤ Q)$ 行目に，整数 $L_i,R_i$ が空白区切りで与えられる.', '## 出力
$T$ 行に渡って出力せよ.
$i$ 行目には，$i$ 個目のテストケースにおける答えを,以下の通りに出力せよ．

```
$ans_j$ を， $j$ 個目の会議における「集まる時間」の最小値とする．
そのとき， $ans_1, ans_2, ... , ans_Q$を空白区切りで出力せよ．
```', '2020-08-28 08:58:52.686693', '2020-08-31 00:21:16.432849', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (25, 'tea001_a', 'Happiness Number', 9, 1, 'A', '98883539-3690-4f76-8dbe-65ebd86686de', 'Milk', 'ポリアンナちゃんは幸せ探しが大好きです。
でも今日は雨。家の中で退屈なポリアンナちゃんは、数字の中から幸せを探すことにしました。

ポリアンナちゃんの幸せとは、整数 $X$ と整数 $Y$ で割り切ることができる数のうち最も小さい数のことをいいます。

ポリアンナちゃんになったつもりで幸せな数を求めてください。', '- $1 \\leq X, Y \\leq 10^6$
- $X$ と $Y$ は互いに素である。', '入力は以下の形式で標準入力から与えられる。
```
$ X \\ Y $
```', '答えを出力してください。', '2020-08-28 10:52:11.916636', '2020-08-28 11:39:38.922768', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (26, 'tea001_b', 'Happiness function', 9, 1, 'B', 'befc4d6a-c9ca-41f0-8451-e589b8158fe4', 'Ceylon', 'ポリアンナちゃんは $P \\leq x \\leq Q$ の範囲で関数を描きます。
X軸との交点( x 座標 $X_i$, y 座標 $Y_i$ )の数が $T$ 以上である関数はポリアンナちゃんを幸せにします。

ポリアンナちゃんがある関数を描いたとき、幸せになれるかどうかを判定してください。

ただし、ポリアンナちゃんが描く関数において接点は存在せず、また x 座標 $ X_i $, $X_{i + 1}$ 間で交点の数も $2$ 個以上存在しません。', '- $ -10^5 \\leq P \\lt Q \\leq 10^5 $
- $ 0 \\leq T \\leq Q - P \\leq 2 \\times 10^5 $
- $ P \\leq X_i \\leq Q $
- $ -10^6 \\leq Y_i \\leq 10^6 $
- $ X_i = P + i $
- $ Y_i \\neq 0 $', '入力は以下の形式で標準入力から与えられる。
```
$ T \\ P \\ Q $
$ X_1 \\ Y_1 $
$ \\vdots $
$ X_{Q - P + 1} \\ Y_{Q - P + 1} $ 
```', 'ポリアンナちゃんが幸せになれるなら `Yes`、幸せになれないなら `No` と出力してください。', '2020-08-28 11:19:34.678399', '2020-08-28 11:37:30.684215', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (27, 'tea001_c', 'Happiness Festival', 9, 1, 'C', 'c6cb3e4b-bd55-4c99-a940-b35e5b137908', 'Darjeeling', '今日は1年に一回のお祭りの日です。ポリアンナちゃんは屋台を引っ張る役割を担っています。

お祭りは縦 $H$ $\\times$ 横 $W$ マスからなる町で開催されます。
ポリアンナちゃんはスタート地点 $ T_{(1, 1)} $ からゴール地点 $ T_{(H, W)} $ へ向かって屋台を引っ張ります。このとき、ポリアンナちゃんは右か下にしか屋台を引っ張ることができません。

お祭りの日はあいにくの雨上がりであったため、路面が荒れているマスとそうでないマスが町の中にあります。1マス移動するとき、移動先のマスが $T_{(i, j)} = 0$ であれば平坦な路面であるためコストはかかりません。しかし、$ T_{(i, j)} = 1$ であれば、路面が荒れているため移動コストが $C$ かかります。

ポリアンナちゃんはなるべくコストがかからないようにしてゴール地点まで屋台を引っ張りたいです。スタート地点からゴール地点まで屋台を引くときにかかるコストの総和の最小値を求めてください。', '- $ 2 \\leq H, W \\leq 10^3 $
- $ 0 \\leq C \\leq 10^3 $
- $ T_{(i, j)} = 0 \\ or \\ 1$
- $ T_{(1, 1)} = 0 $', '$ H \\ W \\ C $
$ T_{(1, 1)} T_{(1, 2)} \\dots T_{(1, W)}$
$ \\vdots $
$ T_{(H, 1)} T_{(H, 2)} \\dots T_{(H, W)}$', '答えを出力してください。', '2020-08-28 11:36:28.882634', '2020-08-28 11:37:32.932987', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (28, 'tea005_e', 'Super Bit Shift', 11, 94, 'E', '2f2d44b9-45c8-4740-809b-7a742255370b', 'Darjeeling', '$2^{(a^b)}$ を求めてください。ただし、答えは非常に大きくなることがあるので、素数 $k$ で割ったあまりを出力してください。
', '- $1 \\leq a,b,k \\leq 10^9+7$
- $a,b,k$ は全て整数である
- $k$ は素数である', '入力は以下の形式で与えられます。

```
$a$ $b$ $k$
```', '答えを一行に出力してください。', '2020-08-31 00:24:16.179349', '2020-12-20 01:46:23.287118', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (29, 'test002_e', 'Do Not Use Segment Tree', 10, 83, 'E', '2512b56f-eecd-4ea6-9521-1ad511a73fd0', 'Earlgray', '長さ $N$ の整数列 $A$ が与えられます。$Q$ 個のクエリを処理してください。

各クエリでは、 $t,l,r,x$ が与えられます。クエリの内容は以下の通りです。

- $t = 0$ のとき : $l$ 以上 $r$ 以下の全ての整数 $i$ について、 $A_i$ を $A_i \\text{ xor } x$ に更新する。
- $t = 1$ のとき : $\\displaystyle\\max_{l \\le i \\le r}A_i$ を出力する。ただし、このクエリは $50$ 回までしか与えられない。

<br> 

Python には定数倍がきついかもしれないので、注意してください。', '- 入力は全て整数
- $1 \\le N \\le 10^5$
- $0 \\le A_i < 2^{32}$
- $1 \\le Q \\le 10^5$
- $0 \\le t \\le 1$
- $1 \\le l \\le r \\le N$
- $0 \\le x < 2^{32}$
- $t = 1$ ならば $x = 0$
- $\\sum t \\le 50$', '```
$N$
$A_1$ $\\dots$ $A_N$
$Q$
$t_1$ $l_1$ $r_1$ $x_1$
$\\vdots$
$t_Q$ $l_Q$ $r_Q$ $x_Q$
```', '$t = 1$ のクエリごとに、答えを改行区切りで出力せよ。', '2020-11-04 03:53:24.702749', '2020-11-17 12:50:52.144219', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (30, 'test002_c', 'NOT IN S', 10, 243, 'C', '09f4f277-d0e4-4495-ba9c-b0dd12402838', 'Darjeeling', '長さ $N$ の英小文字のみからなる文字列 $S$ が与えられます。
次の条件を満たす文字列 $T$ のうち、**長さが最小**のものを出力してください。そのようなものが複数ある場合は**辞書順最小**のものを出力してください。
- $T$ は英小文字のみからなる長さ $1$ 以上の文字列である。
- $T$ は $S$ の**部分文字列**ではない。
  - ここで $S$ の**部分文字列**とは、$S$ の連続した区間を取り出して構成出来るような文字列を指す。

なお、$X = x_1x_2\\cdots x_n, Y = y_1y_2\\cdots y_m$ を二つの異なる文字列とするとき、$Y$ が $X$ の接頭辞であるか、$j$ を $x_j \\neq y_j$ であるような最小の整数として $x_j > y_j$ であるとき、かつその場合に限り $X$ は $Y$ より辞書順で大きいという。', '- $1 \\le N \\le 10^5$
- $S$ は英小文字のみからなる。
- $N$ は整数', '```
$N$
$S$
```', '条件を満たす文字列 $T$ のうち、**長さが最小**のものを出力してください。そのようなものが複数ある場合は**辞書順最小**のものを出力してください。', '2020-11-05 04:21:50.613206', '2020-11-21 09:39:57.717421', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (31, 'test002_f', 'Cheat and Nim', 10, 245, 'F', '9a8b9e34-9125-4d44-825e-824f0d31eae7', 'Flavor', '高橋くんと青木くんは Nim で遊んでいる。
$N$ 個の山にそれぞれ $a_i$ 個の石が置かれていて、高橋くんを先手として交互にひとつの山から好きなだけ石を取っていく。石がどの山からも取れなくなった方が負けである。
さて、高橋くんにどうしても勝ちたい青木くんは、ゲームを始める前に山に少し細工をすることにした。具体的には、青木くんは**1回だけ**以下の操作ができる。
- $N$ 個の山の中から山 $i,j(i\\neq j)$ を選び、山 $i$ から山 $j$ へ石を非負整数個移動させる。山にある石の個数は負にはできないが、$0$ にすることは許される。

青木くんは、ゲームの前にこの操作を行うことで高橋くんに勝つことができるか判定せよ。', '- $1\\leq N\\leq 3000$
- $1\\leq A_i\\leq 10^{18}$', '```
$N$
$A_1\\ A_2\\cdots A_N$
```', '青木くんが勝てる場合は `Yes` を、勝てない場合は `No` を1行に出力せよ。', '2020-11-05 14:15:36.264729', '2020-11-22 16:20:28.339834', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (32, 'test002_b', '2D sort', 10, 12, 'B', '2c8195c4-56e0-40bc-af68-2603a179e425', 'Benihuki', '$ N^2 $ 個の整数 $ A_i $ が与えられます。
$ A$ の要素をそれぞれ1つずつ含み、次の条件を満たす $N$ 行 $N$ 列の行列 $B$ を構成してください。
ただし、条件を満たす行列 $B$ が複数ある場合は、 **$abs(B_{i, j + 1} - B_{i + 1, j})$ の総和が最小となるようなもの** を出力してください。

+ $B_{i, j + 1} \\lt B_{i + 1, j} $ $(1 \\leq i \\lt N,\\ 1 \\leq j \\lt N)$
+ $B_{i, j} \\lt B_{i + 1, j} $ $(1 \\leq i \\lt N,\\ 1 \\leq j \\leq N)$
+ $B_{i, j} \\lt B_{i, j + 1} $ $(1 \\leq i \\leq N,\\ 1 \\leq j \\lt N)$', '+ $ 2 \\leq N \\leq 10^3 $
+ $ 0 \\leq A_i \\leq 10^9 $
+ $ A_i \\neq A_j (i \\neq j) $', '入力は以下の形式で標準入力から与えられます。
```
$ N $
$ A_1 \\ A_2 \\ \\ldots \\ A_{N^2} $
```', '$N$ 行 $N$ 列の行列 $B$ を以下の形式で標準出力に出力してください。
ただし、条件を満たす行列 $B$ が複数ある場合は $abs(B_{i, j + 1} - B_{i + 1, j})$ の総和が最小となるようなものを出力してください。
```
$ B_{1, 1} \\ \\dots \\ B_{1, N} $
$ \\vdots $
$ B_{N, 1} \\ \\dots \\ B_{N, N} $
```', '2020-11-05 14:55:04.800293', '2020-11-21 11:00:56.250453', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (33, 'test002_d', 'Yummy Teablend', 10, 244, 'D', 'c3d6bd34-04f2-4771-9e2f-13753f5610a8', 'Darjeeling', '物理好きさんは茶葉を $N$ 種類持っています。
$i$ 種類目の茶葉は美味しさが $A_i$ 、苦さが $B_i$ です。
物理好きさんは、この中からより多くの種類の茶葉をブレンドして、お茶を作りたいです。
ブレンドしたお茶の美味しさは、茶葉の種類ごとの美味しさの総和になります。
ブレンドしたお茶の苦さは、茶葉の種類ごとの苦さの総和になります。

物理好きさんは、ブレンドの結果、「お茶の美味しさ $\\ge$ お茶の苦さ」となるようにしたいです。
物理好きさんは最大で何種類の茶葉を使うことが出来ますか?', '$1 \\le N \\le 100$
$1 \\le A_i , B_i \\le 100$', '$N$
$A_1\\ A_2\\ ...\\ A_N$
$B_1\\ B_2\\ ...\\ B_N$', '解を1行に出力して下さい。', '2020-11-08 10:41:09.907695', '2020-11-23 08:00:35.567798', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (34, 'test002_a', 'carpet', 10, 58, 'A', 'bd9cbb6c-cb08-431a-bb98-ad4d3acd1c0c', 'Assam', 'あなたは幅 $1$ m 長さ $W$ m の廊下にカーペットを敷こうとしています。（廊下の幅とカーペットの幅は同じで、それぞれは重ねて敷いてはいけません）
カーペットは $N$ 枚あり、$i$ 番目の長さは $A_i$ mです。
あなたは魔法を使って、以下の操作を高々 $K$ 回行います。
+ カーペットをひとつ選び、その長さを $1$ m にする。

<br>
廊下に敷けるカーペットの枚数の最大値を求めてください。', '+ $1 \\leq N \\leq 2 \\times 10^{ 5 }$
+ $1 \\leq W \\leq 10^{ 9 }$
+ $1 \\leq A_i \\leq 200$ $(1 \\leq i \\leq N)$
+ $1 \\leq K \\leq N$
+ 入力は整数値で与えられます。', '入力は以下の形式で標準入力から与えられます。
```
$N \\ \\ W \\ K$
$A_1 \\ A_2 \\ldots \\ A_N$
```', 'あなたが廊下に敷ける絨毯の枚数の最大値を出力してください。', '2020-11-08 11:53:10.672848', '2020-11-22 01:29:36.004294', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (35, 'tea005_f', 'OR or XOR', 11, 159, 'F', '8d380f61-452e-4841-a931-bf6d0240ac04', 'Earlgray', '長さ $N$ の数列 $A_1,A_2,...,A_N$ があります。この数列に対し、次の操作を行います。
1. まず、変数 $x$ を持ちます。これは最初 $0$ です。また、変数 $i$ を持ちます。これは最初 $1$ です。
2. $x$ を、「 $x$ と $A_i$ のbitごとの論理和」もしくは「 $x$ と $A_i$ のbitごとの排他的論理和」で置き換えます。
3. もし、 $i=N$ なら操作を終了します。そうでなければ $i$ に $1$ を足し、操作2に戻ります。

ここで、 $N$ 回の操作 2 でそれぞれ $x$ をどちらで置き換えるかによって、異なる操作を $2^N$ 通り考えることができます。この全てに対する最終的な $x$ の総和を $10^9+7$ で割った余りを出力してください。', '・入力は全て整数
・$1\\leq N\\leq 2×10^5$
・$0\\leq A_i\\leq 10^{18}\\ (1\\leq i\\leq N)$', '入力は以下の形式で与えられます。
```
$N$
$A_1\\ A_2\\ ...\\ A_N$
```', '全ての異なる操作の方法に対する、最終的な $x$ の値の総和を $10^9+7$ で割った余りを 1 行に出力してください。', '2020-11-28 09:41:51.017301', '2020-12-11 11:40:41.911974', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (36, 'tea005_d', 'Factorial Multiplication', 11, 212, 'D', 'bdc05377-df74-4825-93f0-232b47d17d60', 'Benihuki', '$n$ 個の整数が与えられます。$i$ 番目の整数は $a_i$ です。

$\\displaystyle \\prod_{i=1}^{n} a_i!$ を $(10^9+7)$ で割った余りを求めてください。  

ただし、一般に $\\displaystyle \\prod_{i=l}^{r} A_i = A_l \\times A_{l + 1} \\times \\dots \\times A_{r}$ とし、$\\displaystyle x!=\\prod_{k=1}^{x} k$ とします。  
', '- $1 \\le n \\le 2 \\times 10^5$
- $1 \\le a_i \\le 2 \\times 10^5$
- 入力はすべて整数', '入力は以下の形式で標準入力から与えられる。
```
$n$
$a_1\\ a_2\\ \\dots\\ a_n$
```
', '$\\displaystyle \\prod_{i=1}^{n} a_i!$ を $(10^9+7)$ で割った余りを一行に出力してください。最後に改行してください。', '2020-11-28 11:57:34.953231', '2020-12-11 14:25:39.565938', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (37, null, 'Rewrite Cards', null, 212, null, 'f81c6d88-eb18-4174-bb20-0adb7aaed6eb', 'Ceylon', '<p>
null 君は $1$ から $n$ までの数字が書かれた $n$ 枚のカードを持ってます。
</p>
<p>
null 君は次の操作を $m$ 回繰り返します。
</p>
<p>

- $j(1 \\le j \\le m)$ 回目の操作の時、$0 \\le i \\le n-1$ を満たす全ての $i$ について $i+1$ が書かれたカードの数字を $b_{(i + a_j) \\bmod n}$ に書き換える。

</p>
<p>
この操作を見に $q$ 人の kichi さんが来ました。$k(1 \\le k \\le q)$ 人目の kichi さんは操作 $s_k$ が始まる直前から操作 $t_k$ が終わった直後まで操作を見ていました。
</p>
<p>

$k$ 人目の kichi さんについて **操作の見始めに** $x_k$ が書かれていたカードには、操作を見終わったとき何が書かれているかを出力してください。

</p>', '- $1 \\le n, m, q \\le 10^3$
- $0 \\le a_i \\le n - 1$
- $b_0, b_1, \\dots, b_{n-1}$ は $1, 2, \\dots, n$ を並べ替えた数列である。
- $1 \\le s_k \\le t_k \\le m$
- $1 \\le x_k \\le n$
- 入力はすべて整数である。', '$n\\ m\\ q$
$a_1\\ a_2\\ a_3\\ \\dots\\ a_m$
$b_0\\ b_1\\ b_2\\ \\dots\\ b_{n - 1}$
$s_1\\ t_1\\ x_1$
$s_2\\ t_2\\ x_2$
$s_3\\ t_3\\ x_3$
$\\vdots$
$s_q\\ t_q\\ x_q$
', '$k(1 \\le k \\le q)$ 人目の kichi さんについて、操作の見始めに $x_k$ が書かれていたカードには、操作を見終わったとき何が書かれているかを出力してください。各 $k$ ごとに改行してください。', '2020-11-28 15:35:59.197717', '2020-12-03 12:19:53.673311', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (38, 'tea005_c', 'x=?', 11, 45, 'C', 'fdf8be7b-f27d-4b2e-acea-b6e3c461a3a9', 'Assam', '正整数 $A,\\ B,\\ C$ が与えられます。
$\\frac{Ax}{B}≥C$ を満たす最小の整数 $x$ を求めてください。', '・$1≤A,\\ B,\\ C≤10^9$
・入力は全て整数', '入力は以下の形式で標準入力から与えられます。
```
A B C
```', '$\\frac{Ax}{B}≥C$ を満たす最小の整数 $x$ を一行に出力してください。', '2020-11-28 16:17:23.123553', '2020-12-20 06:14:02.223096', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (39, null, ':fake:', null, 45, null, '798ad05d-0659-4bca-96fd-cdab69dbf4b2', 'Milk', '正整数 $A,\\ B,\\ C$ が与えられます。
$A/B×x≥C$ を満たす最小の整数 $x$ を求めてください。', '・$1≤A,\\ B,\\ C≤10^9$
・入力は全て整数', '2 3 4', '6', '2020-11-28 16:17:24.094658', '2020-11-28 19:36:13.983736', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (40, null, 'tea005-a', null, 11, null, '36555cbe-690c-41cf-b6c8-8420ba2f852c', 'Milk', 'a', 'a', 'a', 'a', '2020-12-02 11:17:38.755168', '2020-12-09 11:27:28.666643', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (41, null, 'tea005-b', null, 1, null, '829166db-e115-4db2-94a7-6c5a0343d9c5', 'Milk', 'a', 'a', 'a', 'a', '2020-12-03 12:19:13.718679', '2020-12-07 09:32:02.619349', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (42, null, 'tea005-c', null, 1, null, 'a8bee301-b6ca-4837-86f3-c879b55c8b9a', 'Milk', 'a', 'a', 'a', 'a', '2020-12-03 12:19:22.441003', '2020-12-03 12:21:44.242917', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (43, 'tea005_b', 'String Equalization', 11, 214, 'B', '6b03d460-d05c-438a-8a40-cc9b960a82e0', 'Assam', '長さ $N$ の文字列 $S,T$ が与えられます。$S$ の左から $i(1 \\le i \\le N)$ 番目の文字を $S_i$ と表します。

あなたは、文字列 $S$ に対して以下の操作を任意の回数行うことができます。

- 整数 $i(1 \\le i \\le N)$ を選ぶ。$S_i$ をアルファベット順で $S_i$ の一つ前か後ろの文字で置き換える。  ただし、`''a''`の一つ前の文字は`''z''`、`''z''`の一つ後ろの文字は`''a''`とする。

このとき、 $S$ と $T$ を等しくするために必要な操作回数の最小値を求めてください。', '- $1 \\le N \\le 10^5$
- $N$ は整数である。
- $|S| = |T| = N$
- $S,T$ は英小文字のみを含む
', '入力は以下の形式で標準入力から与えられます。
```
$N$
$S$
$T$
```
', '必要な操作回数の最小値を出力してください。', '2020-12-06 14:26:29.307154', '2020-12-20 08:33:45.835894', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (44, 'tea005_a', 'OR and XOR', 11, 159, 'A', '77425048-103f-4298-8b2a-31c9aeb24c86', 'Milk', '整数 $N$ が与えられます。また、変数 $X,Y$ があります。これは最初どちらも $0$ です。
次の操作を行います。
・ $X$ を「 $X$ と $N$ のbitごとの論理和」で置き換え、 $Y$ を「 $Y$ と $N$ のbitごとの排他的論理和」で置き換える。
操作を行った後の $X$ と $Y$ をそれぞれ求めてください。
<details>
<summary>ヒント</summary>「 $A$ と $B$ のbitごとの論理和」および「 $A$ と $B$ のbitごとの排他的論理和」は $A=0$ のときどちらも $A+B$ に等しくなります。
</details>', '・$N$は整数
・$1\\leq N\\leq 10$', '入力は以下の形式で与えられます。
```
$N$
```', '1 行目に操作後の $X$ の値を、 2 行目に操作後の $Y$ の値をそれぞれ出力してください。', '2020-12-08 14:09:09.069733', '2020-12-11 11:39:00.913621', null);
INSERT INTO p6jav_cafecoder_prod.problems (id, slug, name, contest_id, writer_user_id, position, uuid, difficulty, statement, constraints, input_format, output_format, created_at, updated_at, deleted_at) VALUES (45, null, 'Banana Tea', null, 12, null, '32c03a9a-0e8d-4cf1-8f10-23f8934d9318', 'Ceylon', '$N$ 房のバナナがあります。
セイロンさんはそれらのバナナの本数が昇順になるように左から右に並び替えました。
しかし、セイロンさんはバナナの本数の総和を知りません。
今わかっている情報は、$i$ 番目のバナナの本数は「 $i$ の公約数の総数」ということだけです。
この情報を頼りにして、$N$  房のバナナの本数の総和を求め、セイロンさんに教えてあげてください。', '+ $1 \\ \\leq \\ N \\ \\leq \\ 10^6$', '入力は以下の形式で標準入力から与えられます。
```
$N$
```', '$N$  房のバナナの本数の総和を出力してください。', '2020-12-21 01:48:40.036819', '2020-12-21 01:48:57.621245', null);