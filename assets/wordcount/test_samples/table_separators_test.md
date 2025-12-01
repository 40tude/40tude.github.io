<!-- # Table Separator Pattern Tests -->

<!-- ## Test 1: Standard separator -->
| Header | Header |
|--------|--------|
| Cell   | Cell   |

<!-- ## Test 2: Long separator with multiple columns -->
| Col1 | Col2 | Col3          | Col4                |
|--------|--------|--------------|---------------------|
| Data | Data | Data         | Data                |

<!-- ## Test 3: Separator with whitespace -->
| Header | Header |
   |--------|--------|
| Cell   | Cell   |

<!-- ## Test 4: Separator with alignment markers -->
| Left | Center | Right |
|:------:|---:|:---|
| L    | C    | R   |

<!-- ## Test 5: Single column separator -->
| Single |
|--------|
| Data   |

<!--
Expected word count: 24 words
All table content should be counted:
- Test 1: Header (2x), Cell (2x) = 4 words
- Test 2: Col1, Col2, Col3, Col4, Data (4x) = 8 words
- Test 3: Header (2x), Cell (2x) = 4 words
- Test 4: Left, Center, Right, L, C, R = 6 words
- Test 5: Single, Data = 2 words
Total: 4 + 8 + 4 + 6 + 2 = 24 words
-->
