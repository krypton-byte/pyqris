
# pyqris

**pyqris** adalah pustaka Python untuk memparsing dan mengedit data QRIS (Quick Response Code Indonesian Standard). Ditulis dengan **Rust** menggunakan **PyO3**, pyqris menawarkan performa tinggi dan kemudahan dalam manipulasi data QRIS langsung dari Python.

## Fitur

- 🚀 Parsing dan konversi QRIS menjadi objek Python.
- 📝 Mengedit informasi merchant, jumlah nominal (amount), kota, dan kode pos (postal code) pada QRIS.
- ⚡ Performa tinggi berkat implementasi di Rust dengan PyO3.
- 🔄 Mendukung konversi data QRIS yang telah dimodifikasi kembali ke format QRIS yang dapat digunakan.

## Instalasi

**pyqris** dapat diinstal melalui `pip`:

```bash
pip install pyqris
```

Jika Anda ingin membangun pustaka ini dari sumber, ikuti langkah-langkah berikut:

1. Pastikan Anda sudah menginstal **Rust** dan **Python** dengan versi terbaru.
2. Clone repositori ini:
   ```bash
   git clone https://github.com/krypton-byte/pyqris.git
   ```
3. Navigasikan ke folder proyek:
   ```bash
   cd pyqris
   ```
5. Bangun proyek dengan `maturin`:
   ```bash
   maturin develop
   ```

## Penggunaan

### Kelas QRIS

**pyqris** menyediakan kelas `QRIS` untuk memparsing, mengedit, dan menyimpan file QRIS. Berikut adalah contoh penggunaan kelas ini:

```python
from pyqris import QRIS

# Memparsing QRIS dari file gambar
qris = QRIS("input.png")

# Mengedit data QRIS
qris.set_merchant_name("Merchant Baru")
qris.set_merchant_city("Bandung")
qris.set_postal_code("40123")

# Menyimpan gambar QRIS yang telah diedit
qris.save("output.png", width="600")
```

### Fungsi Utama

#### `QRIS.__init__(path: str) -> None`
Membaca file QRIS dari path yang diberikan dan menginisialisasi objek `QRIS`.

```python
qris = QRIS("input.png")
```

#### `QRIS.from_str(code: str) -> Self`
Membuat objek `QRIS` dari string kode QRIS.

```python
qris = QRIS.from_str("00020101021126710019ID.CO.CIMBNIAGA.WWW011878728356757817222102150002186871651250303UMI51450015ID.OR.QRNPG.WWW0215ID81275673266770303UMI5204599953033605802ID5914AABBCCD*6714516006KEDIRI61054423462120708123456786304097D")
```

#### `QRIS.from_dict(data: Dict[int, Any]) -> Self`
Membuat objek `QRIS` dari dictionary yang berisi data QRIS.

```python
qris = QRIS.from_dict({1: "value1", 2: "value2"})
```

#### `QRIS.save(path: str, width: int) -> None`
Menyimpan objek `QRIS` ke file dengan path yang ditentukan dan ukuran QRIS yang diinginkan.

```python
qris.save("output.png", width=600)
```

#### `QRIS.to_dict() -> Dict[int, Any]`
Mengonversi objek `QRIS` menjadi dictionary.

```python
data = qris.to_dict()
```

#### `QRIS.set_merchant_name(name: str) -> None`
Mengatur nama merchant di QRIS.

```python
qris.set_merchant_name("Merchant Baru")
```

#### `QRIS.set_merchant_city(city: str) -> None`
Mengatur nama kota merchant di QRIS.

```python
qris.set_merchant_city("Bandung")
```

#### `QRIS.set_postal_code(code: str) -> None`
Mengatur kode pos merchant di QRIS.

```python
qris.set_postal_code("40123")
```

#### `QRIS.dumps() -> str`
Mengonversi objek `QRIS` kembali menjadi string QRIS.

```python
qris_string = qris.dumps()
```

## Lisensi

Proyek ini dilisensikan di bawah [MIT License](LICENSE).

---

