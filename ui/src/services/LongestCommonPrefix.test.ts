import { temptest } from './LongestCommonPrefix';

test('countUniqueSubstring', () => {
  temptest('f_1 * ((a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) - a_0[1])');
  temptest(
    'f_7 * ((a_3 * 0xab5e5b874a68de7b3d59fbdc8c9ead497d7a0ab23850b56323f2486d7e11b63 + (a_1 + f_1) * 0x31916628e58a5abb293f0f0d886c7954240d4a7cbf7357368eca5596e996ab5e + (a_2 + f_2) * 0x7c045d5f5e9e5a6d803952bbb364fdfa0a3b71a5fb1573519d1cf25d8e8345d + f_3) * (a_3 * 0xab5e5b874a68de7b3d59fbdc8c9ead497d7a0ab23850b56323f2486d7e11b63 + (a_1 + f_1) * 0x31916628e58a5abb293f0f0d886c7954240d4a7cbf7357368eca5596e996ab5e + (a_2 + f_2) * 0x7c045d5f5e9e5a6d803952bbb364fdfa0a3b71a5fb1573519d1cf25d8e8345d + f_3) * (a_3 * 0xab5e5b874a68de7b3d59fbdc8c9ead497d7a0ab23850b56323f2486d7e11b63 + (a_1 + f_1) * 0x31916628e58a5abb293f0f0d886c7954240d4a7cbf7357368eca5596e996ab5e + (a_2 + f_2) * 0x7c045d5f5e9e5a6d803952bbb364fdfa0a3b71a5fb1573519d1cf25d8e8345d + f_3) * (a_3 * 0xab5e5b874a68de7b3d59fbdc8c9ead497d7a0ab23850b56323f2486d7e11b63 + (a_1 + f_1) * 0x31916628e58a5abb293f0f0d886c7954240d4a7cbf7357368eca5596e996ab5e + (a_2 + f_2) * 0x7c045d5f5e9e5a6d803952bbb364fdfa0a3b71a5fb1573519d1cf25d8e8345d + f_3) * (a_3 * 0xab5e5b874a68de7b3d59fbdc8c9ead497d7a0ab23850b56323f2486d7e11b63 + (a_1 + f_1) * 0x31916628e58a5abb293f0f0d886c7954240d4a7cbf7357368eca5596e996ab5e + (a_2 + f_2) * 0x7c045d5f5e9e5a6d803952bbb364fdfa0a3b71a5fb1573519d1cf25d8e8345d + f_3) - (a_0[1] * 0x2cc057f3fa14687acc59ffd00de864434543705f35e98ab5c6de463cd1404e6b + a_1[1] * 0x32e7c439f2f967e55fd72b55df208385fadbf8ae7ae24796171840417cab7576 + a_2[1] * 0x2eae5df8c3115969f461778abf6c91fa1403db6f50302040942645bd7d4464e0))'
  );
  // temptest('banana');

  // const d = countUniqueSubstring('ababbab');
  // expect(d).toBe(15);

  // const string = "HelloHelloHello Hello HelloHello";
  // const string =
  //   'f_1 * ((a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) - a_0[1])';
  // const pattern = findRepeatPattern(string);
  // console.log(pattern); // Hello
  // Test
  // const s = "apple banana apple apple banana orange orange apple";
  //   const s =
  //     'f_1 * ((a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) * (a_0 + f_0) - a_0[1])';
  // console.log(findRepeatPatterns(s));  // { apple: 4, banana: 2, orange: 2 }
});
