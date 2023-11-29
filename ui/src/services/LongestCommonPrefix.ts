
function createSuffixArray(s: string) {
  const suffixes = [];
  for (let i = 0; i < s.length; i++) {
    suffixes.push({ index: i, suffix: s.slice(i) });
  }
  suffixes.sort((a, b) => a.suffix.localeCompare(b.suffix));
  // console.log(suffixes);
  return suffixes.map((s) => s.index);
}

function createLCPArray(s: string, suffixArray: number[]) {
  const lcpArray = new Array(s.length).fill(0);
  const inverseSuffixArray = new Array(s.length).fill(0);
  for (let i = 0; i < s.length; i++) {
    inverseSuffixArray[suffixArray[i]] = i;
  }
  let k = 0;
  for (let i = 0; i < s.length; i++) {
    if (inverseSuffixArray[i] == s.length - 1) {
      k = 0;
      continue;
    }
    const j = suffixArray[inverseSuffixArray[i] + 1];
    while (i + k < s.length && j + k < s.length && s[i + k] == s[j + k]) {
      k++;
    }
    lcpArray[inverseSuffixArray[i] + 1] = k; // change here
    if (k > 0) {
      k--;
    }
  }
  return lcpArray;
}

export function temptest(s: string) {
  // Test
  // let s = "banana";
  const suffixArray = createSuffixArray(s);
  console.log(suffixArray); // [5, 3, 1, 0, 4, 2]

  const lcpArray = createLCPArray(s, suffixArray);
  console.log(lcpArray); // [1, 0, 3, 0, 0, 2]

  const arr = lcpArray.map((val, index) => [val, suffixArray[index]]);
  arr.sort((a, b) => b[0] - a[0]);
  // console.log(arr);

  for (let i = 0; i < 10; i++) {
    // const suffix = s.slice(suffixArray[i]);
    // const lcp = lcpArray[i];
    // console.log(suffix, lcp);
    console.log(arr[i], s.slice(arr[i][1], arr[i][1] + arr[i][0]));
  }

  let maxLen = 0;
  let idx = -1;

  const lcp = lcpArray;
  const sa = suffixArray;

  for (let i = 0; i < s.length; i++) {
    if (lcp[i] > maxLen && sa[i] + lcp[i] < sa[i - 1]) {
      maxLen = lcp[i];
      idx = sa[i];
    }
  }

  const common = maxLen > 0 ? s.substr(idx, maxLen) : '';

  const escCommon = escapeRegExp(common).trim();
  console.log(escCommon);
  const re = new RegExp(`((${escCommon} \\* )+)${escCommon}`, 'g');
  console.log(re);
  const matches = s.matchAll(re);
  let ss = s;
  // console.log(matches?.map((m) => m.groups));
  for (const match of matches) {
    console.log(match);
    const count = match[1].length / match[2].length + 1;
    ss = ss.replace(match[0], `${common.trim()}^${count}`);
    console.log(ss);
    // console.log(match.index)
  }
}

function escapeRegExp(string: string) {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
