# Dokumentasi Proyek

## Deskripsi Proyek

Proyek ini adalah aplikasi Rust yang menyediakan API untuk manajemen pengguna.
API ini memungkinkan Anda untuk membuat, memperbarui, menghapus, dan mengambil
informasi pengguna.

## Bagaimana Cara Kerjanya

Aplikasi ini menggunakan Rust, sebuah bahasa pemrograman yang menekankan pada
kecepatan dan keselamatan memori. API ini dirancang untuk bekerja dengan
database untuk menyimpan dan mengambil informasi pengguna.

## Fitur

1. **Membuat Pengguna Baru**: API ini memungkinkan Anda untuk membuat pengguna
   baru dengan detail seperti nama, email, dan kata sandi.
2. **Memperbarui Informasi Pengguna**: Anda dapat memperbarui informasi pengguna
   yang ada seperti nama dan email.
3. **Menghapus Pengguna**: Anda dapat menghapus pengguna dari sistem.
4. **Mengambil Informasi Pengguna**: Anda dapat mengambil detail pengguna
   seperti nama dan email.

## Cara Menggunakan

Untuk menggunakan API ini, Anda perlu mengirim permintaan HTTP ke endpoint yang
sesuai. Misalnya, untuk membuat pengguna baru, Anda perlu mengirim permintaan
POST ke `/users` dengan detail pengguna dalam badan permintaan.

## Persyaratan Sistem

Untuk menjalankan aplikasi ini, Anda perlu memiliki Rust dan Cargo (manajer
paket Rust) diinstal di sistem Anda. Anda juga perlu memiliki database yang
kompatibel.

## Cara Menjalankan

Untuk menjalankan aplikasi ini, buka terminal dan navigasikan ke direktori
proyek. Kemudian, jalankan perintah berikut:

```bash
cargo run
```

Ini akan membangun dan menjalankan aplikasi Anda.

## Kontribusi

- [ARusDian](https://github.com/ARusDian) - 11211005
- [not-xygen](https://github.com/not-xygen) - 11201020

Kontribusi selalu diterima! Silakan buat Pull Request dengan perubahan Anda dan
kami akan meninjau secepatnya.

## Lisensi

Proyek ini berlisensi di bawah MIT License. Lihat file `LICENSE` untuk detail
lebih lanjut.
