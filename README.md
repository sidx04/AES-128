# AES Round Function

## 1. `Cipher` function

### 1.1 Cipher Outline

```
Cipher(byte in[4*Nb], byte out[4*Nb], word w[Nb*(Nr+1)]):
begin
    byte state[4,Nb]

    state = in

    AddRoundKey(state, w[0, Nb-1])

    for round = 1 step 1 to Nr–1
        SubBytes(state)
        ShiftRows(state)
        MixColumns(state)
        AddRoundKey(state, w[round*Nb, (round+1)*Nb-1])
    end for

    SubBytes(state)
    ShiftRows(state)
    AddRoundKey(state, w[Nr*Nb, (Nr+1)*Nb-1])

    out = state
end
```

### 1.2 State Array input and output

|    input bytes    |     |     State array     |     |     output bytes      |
| :---------------: | :-: | :-----------------: | :-: | :-------------------: |
| in₀ in₄ in₈ in₁₂  |     | s₀‚₀ s₀‚₁ s₀‚₂ s₀‚₃ |     | out₀ out₄ out₈ out₁₂  |
| in₁ in₅ in₉ in₁₃  |  →  | s₁‚₀ s₁‚₁ s₁‚₂ s₁‚₃ |  →  | out₁ out₅ out₉ out₁₃  |
| in₂ in₆ in₁₀ in₁₄ |     | s₂‚₀ s₂‚₁ s₂‚₂ s₂‚₃ |     | out₂ out₆ out₁₀ out₁₄ |
| in₃ in₇ in₁₁ in₁₅ |     | s₃‚₀ s₃‚₁ s₃‚₂ s₃‚₃ |     | out₃ out₇ out₁₁ out₁₅ |
