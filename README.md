# Tempa

**A lightweight, Compose-first deployment platform for a single Docker server.**

Tempa adalah platform self-hosted untuk mengelola deployment aplikasi berbasis Docker Compose melalui antarmuka dan workflow yang lebih terstruktur.

Tempa dirancang untuk developer yang menjalankan beberapa aplikasi pada satu server atau VPS dan menginginkan pengalaman deployment yang lebih nyaman tanpa harus membangun platform sekompleks Kubernetes, Coolify, atau platform cloud lainnya.

Tempa tidak berusaha mengambil alih seluruh server. Tempa bekerja bersama infrastruktur Docker yang sudah ada dan tetap menghormati resource yang dikelola langsung oleh pengguna.

> **Tempa is a cooperative control plane for your Docker server.**

---

## Status

Tempa masih berada pada tahap perancangan dan pengembangan awal.

Dokumentasi ini menjelaskan arah produk, batasan MVP, dan keputusan teknis utama yang menjadi dasar pengembangan Tempa v2.

---

## Motivation

Menjalankan aplikasi menggunakan Docker Compose pada satu server sebenarnya cukup sederhana. Namun, ketika jumlah aplikasi bertambah, proses operasional mulai tersebar di banyak tempat:

* repository Git yang berbeda;
* file Compose yang berbeda;
* environment variable yang dikelola manual;
* perintah deployment yang dijalankan melalui SSH;
* pencarian container dan log secara manual;
* pengecekan status service satu per satu;
* konfigurasi volume dan network yang sulit dilacak;
* tidak adanya riwayat deployment yang terpusat.

Berbagai platform seperti Coolify dan Dokploy telah menyelesaikan banyak permasalahan tersebut. Namun, platform tersebut juga memiliki cakupan fitur yang lebih luas daripada kebutuhan deployment pada sebuah server pribadi.

Tempa dibuat sebagai eksplorasi untuk membangun platform deployment yang:

* sederhana;
* developer-centric;
* mudah dipahami;
* tetap memberi kontrol terhadap Docker;
* tidak menyembunyikan Docker Compose;
* dapat hidup berdampingan dengan infrastruktur yang sudah tersedia.

---

## Product Positioning

Tempa adalah **self-hosted deployment platform untuk satu Docker server**.

Tempa menyediakan control plane untuk mengelola lifecycle aplikasi berbasis Docker Compose, mulai dari membaca konfigurasi aplikasi sampai menjalankan deployment dan menampilkan status runtime.

Tempa bukan pengganti Docker Compose. Tempa menggunakan Docker Compose sebagai kontrak deployment utama.

```text
Git Repository
      │
      ▼
Compose Discovery
      │
      ▼
Configuration Validation
      │
      ▼
Environment Generation
      │
      ▼
Docker Compose Build
      │
      ▼
Docker Compose Up
      │
      ▼
Health Verification
      │
      ▼
Active Deployment
```

---

## Core Principles

### 1. Compose-first

Docker Compose merupakan satu-satunya happy path pada MVP.

Pengguna mendeskripsikan aplikasi, service, volume, network, health check, dan dependency menggunakan Compose. Tempa tidak membuat format deployment baru yang menggantikan Compose.

### 2. Single-server first

Tempa dirancang terlebih dahulu untuk satu Docker host.

Dukungan cluster, scheduler lintas server, dan orkestrasi terdistribusi tidak termasuk dalam fokus awal.

### 3. Docker remains visible

Tempa tidak mencoba menyembunyikan Docker sepenuhnya.

Konsep seperti service, container, image, volume, network, log, dan health check tetap terlihat di dalam platform.

### 4. Cooperative, not exclusive

Tempa tidak mengasumsikan bahwa seluruh resource Docker pada host dimiliki oleh Tempa.

Pengguna tetap dapat menjalankan container, network, volume, dan Compose project lain di luar Tempa.

### 5. Host as runtime source of truth

PostgreSQL menyimpan konfigurasi dan riwayat yang dikelola oleh Tempa, tetapi kondisi aktual Docker tetap dibaca dari Docker Engine.

Tempa harus mampu mengenali perbedaan antara desired state yang tersimpan dan actual state pada host.

### 6. Explicit over magical

Tempa menghindari terlalu banyak deteksi otomatis dan abstraksi tersembunyi.

Konfigurasi yang eksplisit lebih diprioritaskan dibanding perilaku otomatis yang sulit diprediksi.

---

## Target Users

Tempa pada awalnya dibuat untuk:

* developer individual;
* homelab owner;
* developer yang menjalankan aplikasi pada VPS pribadi;
* tim kecil dengan satu shared Docker server;
* pengguna yang sudah memahami dasar Docker Compose.

Tempa bukan platform SaaS multi-tenant dan bukan cloud hosting provider.

---

## MVP Scope

MVP Tempa berfokus pada workflow deployment aplikasi Docker Compose pada satu server.

### Project Management

Pengguna dapat:

* membuat project;
* melihat daftar project;
* mengubah metadata project;
* menghapus project;
* mengelompokkan beberapa aplikasi dalam satu project.

Contoh struktur:

```text
Project: Personal Infrastructure
├── Application: Portfolio
├── Application: Internal API
├── Application: Monitoring
└── Application: Automation Worker
```

### Application Management

Setiap application memiliki:

* nama;
* identifier atau slug;
* repository source;
* branch;
* lokasi file Compose;
* environment variables;
* deployment history;
* status runtime;
* konfigurasi network;
* konfigurasi volume;
* informasi service yang ditemukan dari Compose.

### Git Repository Source

Pada MVP, aplikasi berasal dari Git repository.

Tempa akan menyimpan konfigurasi seperti:

```text
Repository URL
Branch
Compose file path
Optional repository credential
```

Tempa kemudian mengambil source code pada saat deployment.

### Docker Compose Discovery

Tempa membaca file Compose dari repository dan menemukan:

* service;
* build context;
* image;
* dependency antarservice;
* port;
* health check;
* volume;
* network;
* environment declaration.

Tempa harus menjalankan validasi menggunakan konfigurasi Compose yang sebenarnya sebelum deployment dilakukan.

Contoh:

```bash
docker compose config
```

Tempa tidak membuat parser Compose lengkap sendiri apabila validasi dapat didelegasikan kepada Docker Compose.

### Environment Variables

Tempa menyimpan environment variable yang dibutuhkan oleh aplikasi dan menyediakan nilainya ketika deployment dijalankan.

Nilai sensitif tidak boleh ditampilkan kembali secara penuh melalui antarmuka setelah disimpan.

Environment dapat diteruskan melalui file environment yang dihasilkan khusus untuk deployment.

### Deployment

Deployment merupakan proses terkontrol yang dijalankan oleh Tempa.

Tahapan deployment secara umum:

1. membuat deployment record;
2. memasukkan deployment ke antrean;
3. mengambil atau memperbarui repository;
4. menentukan commit yang akan dideploy;
5. menemukan file Compose;
6. menghasilkan konfigurasi environment;
7. memvalidasi konfigurasi Compose;
8. membangun atau menarik image;
9. menjalankan Docker Compose;
10. memeriksa kondisi service;
11. menandai deployment berhasil atau gagal;
12. memperbarui status aplikasi.

Perintah utama yang digunakan dapat mencakup:

```bash
docker compose config
docker compose pull
docker compose build
docker compose up -d
docker compose ps
docker compose logs
```

Perintah konkret dapat berbeda berdasarkan konfigurasi service.

### Deployment Queue

Deployment dijalankan melalui antrean yang disimpan di PostgreSQL.

Tempa tidak membutuhkan Redis pada MVP.

Deployment untuk application yang sama harus dijalankan secara berurutan untuk mencegah race condition.

```text
Application A
├── Deployment A1 ── running
├── Deployment A2 ── queued
└── Deployment A3 ── queued
```

Deployment untuk application yang berbeda dapat diproses secara paralel apabila kapasitas worker memungkinkan.

### Deployment History

Setiap deployment menyimpan informasi seperti:

* identifier;
* application;
* commit SHA;
* branch;
* trigger;
* waktu dibuat;
* waktu mulai;
* waktu selesai;
* status;
* error summary;
* deployment log.

Status dasar:

```text
queued
preparing
building
deploying
verifying
succeeded
failed
cancelled
```

### Runtime Status

Tempa membaca status aktual dari Docker Engine dan menampilkan:

* service;
* container;
* image;
* state;
* health status;
* uptime;
* exposed port;
* attached network.

Container bukan entity utama yang dibuat langsung oleh pengguna. Container merupakan runtime resource yang dihasilkan oleh sebuah deployment.

### Logs

Pengguna dapat melihat:

* deployment log;
* output build;
* error deployment;
* log service;
* log container.

Log runtime dibaca dari Docker Engine dan tidak harus seluruhnya disimpan secara permanen di PostgreSQL.

### Volumes

Tempa menampilkan volume yang digunakan oleh application berdasarkan definisi Compose dan kondisi aktual Docker.

MVP tidak berusaha menjadi sistem backup volume atau file manager.

### Docker Networks

Tempa menampilkan network yang didefinisikan dalam Compose dan network yang terpasang pada container hasil deployment.

Dukungan terhadap Docker external network merupakan fitur penting dalam arah pengembangan Tempa.

Contoh Compose:

```yaml
services:
  app:
    image: example/app:latest
    networks:
      - proxy
      - database

networks:
  proxy:
    external: true

  database:
    external: true
```

Dalam model ini:

* external network dibuat di luar Tempa;
* host Docker menjadi source of truth untuk ketersediaan network;
* Tempa tidak menghapus external network;
* Tempa memvalidasi bahwa network tersedia sebelum deployment;
* pengguna dapat memilih network host yang akan digunakan aplikasi;
* perubahan attachment network harus tercermin pada konfigurasi deployment.

External network memungkinkan application yang dikelola Tempa berkomunikasi dengan infrastruktur yang sudah ada, seperti:

* reverse proxy;
* database bersama;
* monitoring stack;
* message broker;
* service internal;
* container yang tidak dikelola Tempa.

---

## Domain Model

Domain utama Tempa terdiri dari:

```text
Project
└── Application
    ├── Source
    ├── Configuration
    ├── Environment Variables
    ├── Domains
    ├── Volumes
    ├── Networks
    └── Deployments
        └── Runtime Resources
```

### Project

Project merupakan kelompok logis untuk beberapa application.

Project tidak direpresentasikan sebagai resource Docker secara langsung.

### Application

Application merupakan unit utama yang dikelola pengguna.

Sebuah application memiliki source, konfigurasi Compose, environment, dan riwayat deployment.

### Deployment

Deployment merupakan immutable execution record dari usaha menjalankan versi tertentu sebuah application.

Perubahan konfigurasi tidak mengubah deployment lama. Deployment baru dibuat setiap kali proses deployment dijalankan kembali.

### Runtime Resource

Runtime resource adalah resource Docker yang dihasilkan oleh deployment, seperti:

* container;
* image;
* Compose project;
* network attachment;
* volume attachment.

Runtime resource dapat berubah atau hilang di luar Tempa sehingga statusnya harus direkonsiliasi dengan Docker Engine.

---

## Architecture

Tempa menggunakan arsitektur **modular monolith**.

Pada MVP, backend API dan deployment worker dapat berjalan dalam satu executable yang sama.

```text
┌─────────────────────────────────────────────┐
│                 Web Browser                 │
└──────────────────────┬──────────────────────┘
                       │ HTTP
                       ▼
┌─────────────────────────────────────────────┐
│                   Tempa                     │
│                                             │
│  ┌───────────────────────────────────────┐  │
│  │ HTTP API                              │  │
│  ├───────────────────────────────────────┤  │
│  │ Application Services                  │  │
│  ├───────────────────────────────────────┤  │
│  │ Deployment Queue and Worker           │  │
│  ├───────────────────────────────────────┤  │
│  │ Git and Compose Integration           │  │
│  ├───────────────────────────────────────┤  │
│  │ Docker Engine Integration             │  │
│  └───────────────────────────────────────┘  │
└───────────────┬────────────────────┬────────┘
                │                    │
                ▼                    ▼
        ┌──────────────┐     ┌───────────────┐
        │ PostgreSQL   │     │ Docker Engine │
        └──────────────┘     └───────────────┘
```

Arsitektur ini dipilih agar:

* deployment awal tetap sederhana;
* transaksi database mudah dikelola;
* tidak membutuhkan message broker tambahan;
* debugging lebih mudah;
* modul dapat dipisahkan di masa depan apabila benar-benar diperlukan.

---

## Technology Stack

### Backend

* Rust
* Axum
* Tokio
* SQLx
* PostgreSQL
* Docker Engine API
* Docker Compose CLI
* Git CLI atau integrasi Git setara

### Frontend

* React
* TypeScript
* TanStack Router
* TanStack Query
* Vite

### Runtime

* Docker Engine
* Docker Compose
* Linux server

### Initial Deployment Model

Tempa dijalankan pada server yang sama dengan application yang dikelolanya.

```text
Docker Host
├── Tempa
├── PostgreSQL
├── Application A
├── Application B
├── Reverse Proxy
└── Existing Infrastructure
```

Pemisahan control plane dan remote agent tidak termasuk dalam MVP pertama.

---

## Data Ownership

Tempa membedakan tiga jenis data.

### Managed State

Data yang dimiliki dan dikelola Tempa:

* project;
* application;
* source configuration;
* environment variable;
* deployment record;
* deployment status;
* deployment log;
* konfigurasi yang dipilih pengguna.

Data ini disimpan dalam PostgreSQL.

### Generated State

Data yang dihasilkan Tempa saat deployment:

* working directory;
* checkout repository;
* generated environment file;
* generated Compose override;
* temporary build output.

Data ini dapat dibangun kembali dan tidak selalu harus disimpan secara permanen.

### External Runtime State

Data yang berada pada Docker host:

* container;
* image;
* volume;
* network;
* Compose project;
* container health;
* runtime log.

Tempa mengobservasi dan menggunakan data tersebut, tetapi tidak menganggap semua resource sebagai miliknya.

---

## Resource Ownership

Tempa harus dapat membedakan resource berdasarkan ownership.

### Managed by Tempa

Resource yang dibuat melalui deployment Tempa dan memiliki label internal Tempa.

Contoh label:

```text
io.tempa.managed=true
io.tempa.project=<project-id>
io.tempa.application=<application-id>
io.tempa.deployment=<deployment-id>
```

### Referenced by Tempa

Resource yang digunakan oleh application tetapi dibuat di luar Tempa.

Contoh:

* external Docker network;
* external volume;
* image pada private registry;
* reverse proxy yang sudah berjalan.

### Unmanaged

Resource Docker lain yang tidak berkaitan dengan Tempa.

Tempa dapat menampilkannya untuk kebutuhan observasi, tetapi tidak boleh mengubah atau menghapusnya tanpa aksi eksplisit.

---

## Security Considerations

Tempa memiliki akses ke Docker socket dan karena itu secara praktis memiliki hak tinggi terhadap host.

Instalasi Tempa harus diperlakukan sebagai aplikasi administratif.

Beberapa prinsip keamanan:

* akses dashboard harus dilindungi autentikasi;
* credential repository harus disimpan dengan aman;
* secret tidak dikembalikan dalam bentuk plaintext;
* environment file sementara harus memiliki permission terbatas;
* log harus menghindari pencetakan secret;
* input path harus divalidasi;
* perintah shell tidak boleh dibangun melalui string interpolation yang tidak aman;
* Docker resource hanya boleh dihapus berdasarkan ownership yang jelas;
* destructive action harus eksplisit.

Tempa tidak menyediakan sandbox keamanan untuk menjalankan Compose file yang tidak dipercaya.

Pengguna yang dapat melakukan deployment pada dasarnya dapat menjalankan workload dengan akses terhadap Docker host.

---

## Development Philosophy

Tempa dikembangkan dengan prinsip:

```text
Make it work.
Make the lifecycle clear.
Make failure observable.
Only then add abstraction.
```

Fitur baru harus menjawab setidaknya salah satu pertanyaan berikut:

* Apakah fitur ini membuat deployment Compose lebih mudah?
* Apakah fitur ini membuat lifecycle aplikasi lebih jelas?
* Apakah fitur ini meningkatkan observability ketika terjadi kegagalan?
* Apakah fitur ini membantu integrasi dengan Docker infrastructure yang sudah ada?
* Apakah manfaatnya sebanding dengan kompleksitas yang ditambahkan?

Apabila tidak, fitur tersebut sebaiknya tidak masuk ke dalam MVP.

---

## Inspirations

Tempa terinspirasi oleh beberapa self-hosted deployment dan container management platform, terutama:

* Portainer;
* Dokploy;
* Coolify;
* Docker Compose.

Tempa tidak bertujuan menjadi clone dari salah satu platform tersebut.

Tempa mengambil inspirasi dari kemudahan pengelolaan container dan deployment workflow, lalu membatasi scope pada kebutuhan aplikasi Compose di satu server.

---

## Contributing

### Testing

Unit test dapat dijalankan tanpa service eksternal:

```bash
cargo test -p backend --lib
```

Integration test membutuhkan PostgreSQL dari Docker Compose dan database khusus testing. Setelah `.env` berisi `DATABASE_URL` yang sesuai, jalankan:

```bash
docker compose up -d db
set -a && source .env && set +a
cargo test --workspace -- --test-threads=1
```

`DATABASE_URL_TEST` dapat digunakan untuk mengarahkan integration test ke database terpisah; variabel ini diprioritaskan daripada `DATABASE_URL`.

Tempa saat ini merupakan personal learning project dan masih berada pada fase eksplorasi.

Panduan kontribusi akan ditambahkan ketika struktur project dan workflow pengembangan sudah cukup stabil.

---

## License

Lisensi project belum ditentukan.
