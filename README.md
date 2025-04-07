📄 README.md

# 🌾 Neural Network untuk Klasifikasi Biji Gandum (Wheat Classification)

Program ini membangun dan menguji sebuah model jaringan saraf tiruan (Neural Network) sederhana yang ditulis dalam bahasa pemrograman Rust. Model ini digunakan untuk mengklasifikasikan jenis biji gandum berdasarkan fitur fisiknya.

## 📦 Dataset

Dataset yang digunakan adalah `seeds_dataset.txt`, yang berisi 7 fitur numerik dan 1 label kelas untuk masing-masing sampel. Terdapat 3 kelas jenis biji gandum (label 1, 2, 3) yang dikonversi menjadi (0, 1, 2).

### Fitur-fitur:
1. Area
2. Perimeter
3. Compactness
4. Kernel Length
5. Kernel Width
6. Asymmetry Coefficient
7. Length of Kernel Groove

## 🧠 Arsitektur Neural Network

- Input Layer: 7 neuron
- Hidden Layer: 10 neuron (ReLU activation)
- Output Layer: 3 neuron (Softmax activation)

## ⚙️ Alur Program

1. Membaca dataset dari file teks
2. Menyusun fitur dan label menjadi array numerik
3. Menginisialisasi bobot secara acak
4. Melakukan forward pass untuk memprediksi label dari satu sampel
5. Menampilkan output berupa probabilitas dan label prediksi

## 🚀 Cara Menjalankan

Pastikan kamu sudah menginstal `Rust` dan `cargo`.

1. Clone atau salin repositori ini
2. Letakkan file `seeds_dataset.txt` di direktori yang sama dengan `main.rs`
3. Jalankan perintah berikut di terminal:
```bash
cargo run
```

4. Output akan menunjukkan:
   - Jumlah data
   - Fitur sampel pertama
   - Output prediksi probabilitas
   - Label prediksi vs label sebenarnya

## 🖼️ Visualisasi Arsitektur

Desain jaringan divisualisasikan menggunakan Graphviz. File `.dot` dapat di-render menjadi PNG:

```bash
dot -Tpng nn_architecture.dot -o nn_architecture.png
```

## 📚 Dependensi

Tambahkan ini di `Cargo.toml`:

```toml
[dependencies]
ndarray = "0.15"
ndarray-rand = "0.14"
```

## 📌 Catatan

- Model ini hanya menggunakan satu sampel untuk forward pass, tidak melakukan pelatihan (training).
- Visualisasi arsitektur dibuat secara manual sesuai dengan ukuran layer dari kode.
```

---

Kalau kamu ingin file README.md ini aku bantu simpan langsung dalam format file, tinggal bilang aja ya! Mau sekalian aku bantu bikin file `.dot` untuk Graphviz juga?
