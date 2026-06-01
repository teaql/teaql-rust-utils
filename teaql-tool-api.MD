# TeaQL Tool API Catalog

## Core

### T::time()

```rust
T::time().now()
T::time().today()
T::time().parse_date(...)
T::time().parse_datetime(...)
T::time().add_days(...)
T::time().add_months(...)
T::time().start_of_day(...)
T::time().end_of_day(...)
T::time().days_between(...)
T::time().to_timezone(...)
```

Implementation candidates:

```text
chrono
time
chrono-tz
```

---

### T::id()

```rust
T::id().uuid()
T::id().uuid_v7()
T::id().ulid()
T::id().nanoid()
T::id().snowflake()
T::id().short()
T::id().with_prefix(...)
```

Implementation candidates:

```text
uuid
ulid
nanoid
```

---

### T::decimal()

```rust
T::decimal().of(...)
T::decimal().zero()
T::decimal().one()

T::decimal().add(...)
T::decimal().sub(...)
T::decimal().mul(...)
T::decimal().div(...)

T::decimal().round(...)
T::decimal().ceil(...)
T::decimal().floor(...)

T::decimal().abs(...)
T::decimal().min(...)
T::decimal().max(...)

T::decimal().percent(...)
T::decimal().ratio(...)
```

Implementation candidates:

```text
rust_decimal
```

---

### T::money()

```rust
T::money().of(...)
T::money().zero(...)

T::money().add(...)
T::money().sub(...)
T::money().mul(...)
T::money().div(...)

T::money().round(...)
T::money().allocate(...)
T::money().same_currency(...)
T::money().format(...)
```

Implementation candidates:

```text
rust_decimal
iso_currency
```

---

### T::text()

```rust
T::text().trim(...)
T::text().lowercase(...)
T::text().uppercase(...)
T::text().capitalize(...)

T::text().normalize(...)
T::text().normalize_whitespace(...)

T::text().to_snake_case(...)
T::text().to_camel_case(...)
T::text().to_pascal_case(...)
T::text().to_kebab_case(...)

T::text().contains(...)
T::text().starts_with(...)
T::text().ends_with(...)

T::text().substring(...)
T::text().truncate(...)
T::text().ellipsis(...)

T::text().split(...)
T::text().join(...)

T::text().mask(...)
T::text().mask_phone(...)
T::text().mask_email(...)

T::text().slugify(...)
```

Implementation candidates:

```text
unicode-normalization
convert_case
deunicode
```

---

### T::json()

```rust
T::json().parse(...)
T::json().stringify(...)
T::json().stringify_pretty(...)

T::json().get(...)
T::json().set(...)
T::json().remove(...)
T::json().has(...)

T::json().merge(...)
T::json().deep_merge(...)

T::json().diff(...)
T::json().patch(...)

T::json().schema_validate(...)
```

Implementation candidates:

```text
serde_json
json-patch
jsonschema
```

---

### T::list()

```rust
T::list().empty()
T::list().of(...)

T::list().len(...)
T::list().first(...)
T::list().last(...)

T::list().contains(...)
T::list().distinct(...)
T::list().reverse(...)

T::list().sort(...)
T::list().sort_by(...)

T::list().take(...)
T::list().skip(...)
T::list().slice(...)

T::list().chunk(...)
T::list().flatten(...)

T::list().map(...)
T::list().filter(...)
T::list().find(...)

T::list().any(...)
T::list().all(...)

T::list().group_by(...)
T::list().count_by(...)

T::list().sum(...)
T::list().avg(...)
T::list().min(...)
T::list().max(...)
```

Implementation candidates:

```text
std
itertools
```

---

### T::map()

```rust
T::map().empty()
T::map().of(...)

T::map().get(...)
T::map().put(...)
T::map().remove(...)

T::map().contains_key(...)

T::map().keys(...)
T::map().values(...)
T::map().entries(...)

T::map().merge(...)
T::map().deep_merge(...)

T::map().pick(...)
T::map().omit(...)
T::map().invert(...)
```

Implementation candidates:

```text
std::collections
```

---

### T::validate()

```rust
T::validate().required(...)
T::validate().not_blank(...)

T::validate().min_length(...)
T::validate().max_length(...)
T::validate().length_between(...)

T::validate().min(...)
T::validate().max(...)
T::validate().range(...)

T::validate().email(...)
T::validate().url(...)
T::validate().phone(...)
T::validate().regex(...)

T::validate().date(...)
T::validate().datetime(...)
T::validate().decimal(...)
T::validate().integer(...)
T::validate().json(...)
```

Implementation candidates:

```text
validator
regex
```

---

### T::regex()

```rust
T::regex().is_match(...)
T::regex().find(...)
T::regex().find_all(...)

T::regex().capture(...)
T::regex().capture_all(...)

T::regex().replace(...)
T::regex().replace_all(...)

T::regex().split(...)
T::regex().escape(...)
T::regex().validate(...)
```

Implementation candidates:

```text
regex
```

---

### T::code()

```rust
T::code().base64_encode(...)
T::code().base64_decode(...)

T::code().hex_encode(...)
T::code().hex_decode(...)

T::code().url_encode(...)
T::code().url_decode(...)

T::code().html_escape(...)
T::code().html_unescape(...)

T::code().json_escape(...)
T::code().json_unescape(...)
```

Implementation candidates:

```text
base64
hex
urlencoding
html_escape
```

---

### T::hash()

```rust
T::hash().sha256(...)
T::hash().sha512(...)
T::hash().blake3(...)

T::hash().crc32(...)
T::hash().checksum(...)

T::hash().stable_json(...)
T::hash().stable_text(...)
```

Implementation candidates:

```text
sha2
blake3
crc32fast
```

---

### T::url()

```rust
T::url().parse(...)
T::url().is_valid(...)

T::url().scheme(...)
T::url().host(...)
T::url().port(...)
T::url().path(...)
T::url().query(...)
T::url().fragment(...)

T::url().join(...)
T::url().set_query_param(...)
T::url().remove_query_param(...)

T::url().normalize(...)
```

Implementation candidates:

```text
url
```

---

### T::email()

```rust
T::email().is_valid(...)
T::email().parse(...)

T::email().local_part(...)
T::email().domain(...)

T::email().normalize(...)
T::email().mask(...)
```

Implementation candidates:

```text
email_address
```

---

### T::phone()

```rust
T::phone().parse(...)
T::phone().is_valid(...)

T::phone().country_code(...)
T::phone().national_number(...)

T::phone().format_e164(...)
T::phone().format_international(...)

T::phone().mask(...)
T::phone().normalize(...)
```

Implementation candidates:

```text
phonenumber
```

---

### T::address()

```rust
T::address().normalize(...)
T::address().split_lines(...)
T::address().join_lines(...)

T::address().country(...)
T::address().postal_code(...)

T::address().mask(...)
```

---

### T::geo()

```rust
T::geo().point(...)

T::geo().distance(...)
T::geo().bearing(...)
T::geo().midpoint(...)

T::geo().within_radius(...)

T::geo().bounding_box(...)
T::geo().polygon_contains(...)

T::geo().geohash(...)
T::geo().decode_geohash(...)
```

Implementation candidates:

```text
geo
geohash
```

---

### T::csv()

```rust
T::csv().read(...)
T::csv().write(...)

T::csv().headers(...)
T::csv().detect_headers(...)

T::csv().to_json_rows(...)
T::csv().from_json_rows(...)
```

Implementation candidates:

```text
csv
```

---

### T::excel()

```rust
T::excel().read(...)
T::excel().sheet_names(...)
T::excel().read_sheet(...)

T::excel().cell(...)
T::excel().rows(...)

T::excel().new_workbook(...)
T::excel().add_sheet(...)
T::excel().write_cell(...)
T::excel().write_rows(...)
```

Implementation candidates:

```text
calamine
rust_xlsxwriter
```

---

### T::image()

```rust
T::image().open(...)
T::image().dimensions(...)

T::image().resize(...)
T::image().thumbnail(...)
T::image().crop(...)

T::image().rotate90(...)
T::image().rotate180(...)

T::image().flip_horizontal(...)
T::image().flip_vertical(...)

T::image().grayscale(...)
T::image().blur(...)

T::image().encode_png(...)
T::image().encode_jpeg(...)
```

Implementation candidates:

```text
image
fast_image_resize
```

---

### T::unit()

```rust
T::unit().kg(...)
T::unit().meter(...)
T::unit().liter(...)

T::unit().bar(...)
T::unit().mpa(...)
T::unit().psi(...)

T::unit().celsius(...)
T::unit().fahrenheit(...)

T::unit().convert(...)
T::unit().same_dimension(...)
```

Implementation candidates:

```text
uom
```

---

### T::crypto()

```rust
T::crypto().hmac_sha256(...)
T::crypto().constant_time_eq(...)

T::crypto().aes_gcm_encrypt(...)
T::crypto().aes_gcm_decrypt(...)

T::crypto().random_bytes(...)
```

Implementation candidates:

```text
aes-gcm
hmac
argon2
```

---

### T::random()

```rust
T::random().bool()
T::random().int(...)
T::random().decimal(...)

T::random().bytes(...)
T::random().string(...)

T::random().pick(...)
T::random().shuffle(...)

T::random().weighted_pick(...)
```

Implementation candidates:

```text
rand
```

---

### T::diff()

```rust
T::diff().text(...)
T::diff().lines(...)
T::diff().words(...)

T::diff().json(...)
T::diff().json_patch(...)

T::diff().list(...)
T::diff().map(...)

T::diff().changed_fields(...)
T::diff().summary(...)
```

Implementation candidates:

```text
similar
json-patch
```

---

### T::template()

```rust
T::template().render(...)
T::template().render_strict(...)

T::template().validate(...)
T::template().variables(...)

T::template().escape_html(...)
T::template().escape_json(...)
```

Implementation candidates:

```text
handlebars
tera
minijinja
```
