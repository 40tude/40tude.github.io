# Tiny Bahdanau encoder-decoder that follows the slide (attention.drawio).
# Trained on a template corpus; "j' aime écrire du code propre en rust ." is held out.
# Produces the attention heatmap of Episode 03 (assets/img22.webp).
#
# Run: uv run --with torch --with matplotlib --with pillow --with numpy python heatmap_toy.py [out.webp]

import itertools
import random
import sys

import numpy as np
import torch
import torch.nn as nn

SEED = 1
random.seed(SEED)
torch.manual_seed(SEED)

# ---------------------------------------------------------------- corpus
subjects = [("j'", "aime", "i", "like"), ("tu", "aimes", "you", "like"),
            ("il", "aime", "he", "likes"), ("elle", "aime", "she", "likes"),
            ("nous", "aimons", "we", "like")]
actions = [("écrire", "writing"), ("lire", "reading"), ("relire", "rereading"), ("tester", "testing")]
adjectives = [("propre", "clean"), ("sale", "dirty"), ("lisible", "readable"),
              ("rapide", "fast"), ("simple", "simple"), (None, None)]
languages = [("rust", "rust"), ("python", "python"), ("go", "go"), ("java", "java"), ("c", "c")]

pairs = []
for (s_fr, v_fr, s_en, v_en), (a_fr, a_en), (adj_fr, adj_en), (l_fr, l_en) in itertools.product(
        subjects, actions, adjectives, languages):
    fr = [s_fr, v_fr, a_fr, "du", "code"] + ([adj_fr] if adj_fr else []) + ["en", l_fr, "."]
    en = [s_en, v_en, a_en] + ([adj_en] if adj_en else []) + ["code", "in", l_en, "."]
    pairs.append((fr, en))

held_out = ("j' aime écrire du code propre en rust .".split(), "i like writing clean code in rust .".split())
train_pairs = [p for p in pairs if p[0] != held_out[0]]
assert len(train_pairs) == len(pairs) - 1


def build_vocab(sentences):
    vocab = {"<start>": 0, "<end>": 1}
    for sentence in sentences:
        for token in sentence:
            vocab.setdefault(token, len(vocab))
    return vocab


fr_vocab = build_vocab(p[0] for p in pairs)
en_vocab = build_vocab(p[1] for p in pairs)
en_itos = {i: t for t, i in en_vocab.items()}


def encode(tokens, vocab):
    return torch.tensor([vocab["<start>"]] + [vocab[t] for t in tokens] + [vocab["<end>"]])


# ---------------------------------------------------------------- model (names of the slide)
class Encoder(nn.Module):
    def __init__(self, vocab_size, emb_dim, n_units):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, emb_dim)   # blue Embedding
        self.gru = nn.GRU(emb_dim, n_units, batch_first=True)

    def forward(self, src_ids):
        output, state = self.gru(self.embedding(src_ids))
        return output, state                                 # (1, seq_len, n_units), (1, 1, n_units)


class Attention(nn.Module):
    def __init__(self, n_units):
        super().__init__()
        self.W1 = nn.Linear(n_units, n_units)                # reads the book
        self.W2 = nn.Linear(n_units, n_units)                # reads the decoder memory
        self.V = nn.Linear(n_units, 1)                       # one neuron

    def forward(self, s_prev, output):
        scores = self.V(torch.tanh(self.W1(output) + self.W2(s_prev).unsqueeze(1)))  # (1, seq_len, 1)
        weights = torch.softmax(scores.squeeze(-1), dim=-1)  # (1, seq_len)
        context = weights.unsqueeze(1) @ output              # (1, 1, n_units)
        return weights, context


class Decoder(nn.Module):
    def __init__(self, vocab_size, emb_dim, n_units):
        super().__init__()
        self.embedding = nn.Embedding(vocab_size, emb_dim)   # red Embedding
        self.attention = Attention(n_units)
        self.gru = nn.GRU(emb_dim + n_units, n_units, batch_first=True)
        self.fc = nn.Linear(n_units, vocab_size)             # fc Dense

    def step(self, y_prev, s_prev, output):
        weights, context = self.attention(s_prev[0], output)
        y_emb = self.embedding(y_prev).view(1, 1, -1)
        gru_in = torch.cat([y_emb, context], dim=-1)         # concat
        _, s = self.gru(gru_in, s_prev)                      # s_t
        logits = self.fc(s[0])                               # (1, vocab_size)
        return logits, s, weights


EMB_DIM, N_UNITS = 16, 32
encoder = Encoder(len(fr_vocab), EMB_DIM, N_UNITS)
decoder = Decoder(len(en_vocab), EMB_DIM, N_UNITS)
params = list(encoder.parameters()) + list(decoder.parameters())
optimizer = torch.optim.Adam(params, lr=3e-3)
loss_fn = nn.CrossEntropyLoss()                              # softmax + cross-entropy

# ---------------------------------------------------------------- training (teacher forcing)
for epoch in range(15):
    random.shuffle(train_pairs)
    total = 0.0
    for fr, en in train_pairs:
        src, tgt = encode(fr, fr_vocab).unsqueeze(0), encode(en, en_vocab)
        output, s = encoder(src)                             # switch 2: s_0 = state
        loss = 0.0
        for t in range(1, len(tgt)):
            logits, s, _ = decoder.step(tgt[t - 1], s, output)   # switch 3: true previous token
            loss = loss + loss_fn(logits, tgt[t:t + 1])
        optimizer.zero_grad()
        loss.backward()
        optimizer.step()
        total += loss.item() / (len(tgt) - 1)
    print(f"epoch {epoch:2d}  mean loss per token {total / len(train_pairs):.4f}")

# ---------------------------------------------------------------- inference on the held-out sentence
fr, _ = held_out
src_tokens = ["<start>"] + fr + ["<end>"]
with torch.no_grad():
    output, s = encoder(encode(fr, fr_vocab).unsqueeze(0))
    y = torch.tensor(en_vocab["<start>"])
    produced, rows = [], []
    for _ in range(20):
        logits, s, weights = decoder.step(y, s, output)
        y = logits.argmax(dim=-1)[0]                         # greedy, green wire
        produced.append(en_itos[y.item()])
        rows.append(weights[0].numpy())
        if y.item() == en_vocab["<end>"]:
            break

print(" ".join(produced))
A = np.stack(rows).T                                         # rows = source tokens, cols = target tokens
np.set_printoptions(precision=2, suppress=True)
print(A)

# ---------------------------------------------------------------- heatmap
if len(sys.argv) > 1:
    import io

    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib.colors import LinearSegmentedColormap
    from PIL import Image

    ramp = ["#ffffff", "#cce3fd", "#9dc8f7", "#6aabf1", "#338ced", "#2370cd", "#1757ac", "#0e3d7e"]
    cmap = LinearSegmentedColormap.from_list("blue_seq", ramp)
    ink, muted = "#1f1f1f", "#6b6b6b"

    fig, ax = plt.subplots(figsize=(8, 8.6), dpi=150)
    im = ax.imshow(A, cmap=cmap, vmin=0, vmax=1, aspect="equal")
    ax.set_xticks(range(len(produced)), produced, rotation=0, color=ink, fontsize=11)
    ax.set_yticks(range(len(src_tokens)), src_tokens, color=ink, fontsize=11)
    ax.xaxis.tick_top()
    ax.set_xlabel("English tokens produced, one per decoder step", color=muted, fontsize=10, labelpad=10)
    ax.xaxis.set_label_position("top")
    ax.set_ylabel("French source tokens (rows of the book)", color=muted, fontsize=10)
    ax.set_xticks(np.arange(-0.5, len(produced)), minor=True)
    ax.set_yticks(np.arange(-0.5, len(src_tokens)), minor=True)
    ax.grid(which="minor", color="white", linewidth=2)
    ax.tick_params(which="both", length=0)
    for spine in ax.spines.values():
        spine.set_visible(False)
    for i in range(A.shape[0]):
        for j in range(A.shape[1]):
            if A[i, j] >= 0.10:
                ax.text(j, i, f"{A[i, j]:.2f}", ha="center", va="center", fontsize=8,
                        color="white" if A[i, j] >= 0.5 else ink)
    cbar = fig.colorbar(im, ax=ax, fraction=0.035, pad=0.03)
    cbar.outline.set_visible(False)
    cbar.ax.tick_params(colors=muted, labelsize=9, length=0)
    cbar.set_label("attention weight", color=muted, fontsize=10)
    fig.tight_layout()
    buf = io.BytesIO()
    fig.savefig(buf, format="png", facecolor="white", bbox_inches="tight", pad_inches=0.3)
    Image.open(buf).convert("RGB").save(sys.argv[1], "WEBP", quality=85)
    print("saved", sys.argv[1])
