https://www.hackerrank.com/contests/software-engineer-prep-kit/challenges/check-palindrome-filter-non-letters/problem?isFullScreen=true

Check Palindrome by Filtering Non-Letters

Given a string containing letters, digits, and symbols, determine if it reads the same forwards and backwards when considering only alphabetic characters (case-insensitive).

Example Input

code = A1b2B!a

Output

1

Explanation

- Step 1: Extract only letters → ['A','b','B','a']
- Step 2: Convert to lowercase → ['a','b','b','a']
- Step 3: Compare sequence forward and backward: 'abba' == 'abba' → true

Input Format

    A string code containing letters (A–Z, a–z), digits (0–9), and symbols

Constraints

    0 <= code.length <= 1000
    For all 0 <= i < code.length: 33 <= ASCII(code[i]) <= 126
    code contains only printable ASCII characters (letters, digits, symbols)

Output Format

    Return a boolean value: 1 if true & 0 if false.

Sample Input 0

Z

Sample Output 0

1

Sample Input 1

abc123cba

Sample Output 1

1

====
Additional tests
palindromes

    abc!def<>?fed&cba  
    46AACC57#tcBTabx0"£$%%*&^&*0xbaTBct#75CCAA64
    B976ED04,MkubXERo:~@%^oREXbukM&40DE679B
    5B8EA2BD-C5Xl75WS:~@%^SW57lX5C!DB2AE8B5
    49C43772%9TXHRhEp:~@%^pEhRHXT9,27734C94
    3F177D57!4V4r8PKo"£$%%*&^&*oKP8r4V4#75D771F3
    5FD5AB64)m07qFxDl,./$lDxFq70m%46BA5DF5
    6B929618&to4khLbW:~@%^WbLhk4ot"816929B6

Not palindromes

    EACC3D0B/FDAIGvSAEACC3D0B/FDAIGvSAB0D3CCAE
    568747E6,yuuVHd7n568747E6,yuuVHd7n6E747865
    DD5A014B%QbFUJct8DD5A014B%QbFUJct8B410A5DD
    E3825639.TxEf11k4E3825639.TxEf11k49365283E
    F385BD44.164N2Pd9F385BD44.164N2Pd944DB583F
    B06B82ED)vykdfRQ7B06B82ED)vykdfRQ7DE28B60B
    91030211%UluNYz5H91030211%UluNYz5H11203019
    7C38BC87!KPtzSuVx7C38BC87!KPtzSuVx78CB83C7