// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data obtained from ICU4C:
//! <https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L264>

use super::{HijriYear, PackedHijriYearData};

pub const STARTING_YEAR: i32 = 1300;

#[allow(clippy::unwrap_used)] // const
pub const DATA: &[PackedHijriYearData] = {
    use calendrical_calculations::gregorian::fixed_from_gregorian as gregorian;
    &[
        HijriYear::try_new(
            1300,
            [
                gregorian(1882, 11, 12),
                gregorian(1882, 12, 12),
                gregorian(1883, 1, 10),
                gregorian(1883, 2, 9),
                gregorian(1883, 3, 10),
                gregorian(1883, 4, 9),
                gregorian(1883, 5, 8),
                gregorian(1883, 6, 7),
                gregorian(1883, 7, 6),
                gregorian(1883, 8, 5),
                gregorian(1883, 9, 3),
                gregorian(1883, 10, 3),
                gregorian(1883, 11, 1),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1301,
            [
                gregorian(1883, 11, 1),
                gregorian(1883, 12, 1),
                gregorian(1883, 12, 31),
                gregorian(1884, 1, 29),
                gregorian(1884, 2, 28),
                gregorian(1884, 3, 28),
                gregorian(1884, 4, 27),
                gregorian(1884, 5, 26),
                gregorian(1884, 6, 25),
                gregorian(1884, 7, 24),
                gregorian(1884, 8, 23),
                gregorian(1884, 9, 21),
                gregorian(1884, 10, 20),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1302,
            [
                gregorian(1884, 10, 20),
                gregorian(1884, 11, 19),
                gregorian(1884, 12, 19),
                gregorian(1885, 1, 18),
                gregorian(1885, 2, 16),
                gregorian(1885, 3, 18),
                gregorian(1885, 4, 17),
                gregorian(1885, 5, 16),
                gregorian(1885, 6, 14),
                gregorian(1885, 7, 14),
                gregorian(1885, 8, 12),
                gregorian(1885, 9, 10),
                gregorian(1885, 10, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1303,
            [
                gregorian(1885, 10, 10),
                gregorian(1885, 11, 8),
                gregorian(1885, 12, 8),
                gregorian(1886, 1, 7),
                gregorian(1886, 2, 5),
                gregorian(1886, 3, 7),
                gregorian(1886, 4, 6),
                gregorian(1886, 5, 5),
                gregorian(1886, 6, 4),
                gregorian(1886, 7, 3),
                gregorian(1886, 8, 2),
                gregorian(1886, 8, 31),
                gregorian(1886, 9, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1304,
            [
                gregorian(1886, 9, 29),
                gregorian(1886, 10, 28),
                gregorian(1886, 11, 27),
                gregorian(1886, 12, 27),
                gregorian(1887, 1, 25),
                gregorian(1887, 2, 24),
                gregorian(1887, 3, 26),
                gregorian(1887, 4, 25),
                gregorian(1887, 5, 24),
                gregorian(1887, 6, 23),
                gregorian(1887, 7, 22),
                gregorian(1887, 8, 21),
                gregorian(1887, 9, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1305,
            [
                gregorian(1887, 9, 19),
                gregorian(1887, 10, 18),
                gregorian(1887, 11, 16),
                gregorian(1887, 12, 16),
                gregorian(1888, 1, 15),
                gregorian(1888, 2, 13),
                gregorian(1888, 3, 14),
                gregorian(1888, 4, 13),
                gregorian(1888, 5, 12),
                gregorian(1888, 6, 11),
                gregorian(1888, 7, 11),
                gregorian(1888, 8, 9),
                gregorian(1888, 9, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1306,
            [
                gregorian(1888, 9, 7),
                gregorian(1888, 10, 7),
                gregorian(1888, 11, 5),
                gregorian(1888, 12, 5),
                gregorian(1889, 1, 3),
                gregorian(1889, 2, 2),
                gregorian(1889, 3, 3),
                gregorian(1889, 4, 2),
                gregorian(1889, 5, 1),
                gregorian(1889, 5, 31),
                gregorian(1889, 6, 30),
                gregorian(1889, 7, 29),
                gregorian(1889, 8, 28),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1307,
            [
                gregorian(1889, 8, 28),
                gregorian(1889, 9, 26),
                gregorian(1889, 10, 26),
                gregorian(1889, 11, 24),
                gregorian(1889, 12, 24),
                gregorian(1890, 1, 22),
                gregorian(1890, 2, 21),
                gregorian(1890, 3, 22),
                gregorian(1890, 4, 21),
                gregorian(1890, 5, 20),
                gregorian(1890, 6, 19),
                gregorian(1890, 7, 18),
                gregorian(1890, 8, 17),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1308,
            [
                gregorian(1890, 8, 17),
                gregorian(1890, 9, 15),
                gregorian(1890, 10, 15),
                gregorian(1890, 11, 14),
                gregorian(1890, 12, 13),
                gregorian(1891, 1, 12),
                gregorian(1891, 2, 10),
                gregorian(1891, 3, 12),
                gregorian(1891, 4, 10),
                gregorian(1891, 5, 10),
                gregorian(1891, 6, 8),
                gregorian(1891, 7, 7),
                gregorian(1891, 8, 6),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1309,
            [
                gregorian(1891, 8, 6),
                gregorian(1891, 9, 4),
                gregorian(1891, 10, 4),
                gregorian(1891, 11, 3),
                gregorian(1891, 12, 3),
                gregorian(1892, 1, 2),
                gregorian(1892, 1, 31),
                gregorian(1892, 2, 29),
                gregorian(1892, 3, 30),
                gregorian(1892, 4, 28),
                gregorian(1892, 5, 27),
                gregorian(1892, 6, 26),
                gregorian(1892, 7, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1310,
            [
                gregorian(1892, 7, 25),
                gregorian(1892, 8, 24),
                gregorian(1892, 9, 22),
                gregorian(1892, 10, 22),
                gregorian(1892, 11, 21),
                gregorian(1892, 12, 21),
                gregorian(1893, 1, 19),
                gregorian(1893, 2, 18),
                gregorian(1893, 3, 19),
                gregorian(1893, 4, 18),
                gregorian(1893, 5, 17),
                gregorian(1893, 6, 15),
                gregorian(1893, 7, 15),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1311,
            [
                gregorian(1893, 7, 15),
                gregorian(1893, 8, 13),
                gregorian(1893, 9, 12),
                gregorian(1893, 10, 11),
                gregorian(1893, 11, 10),
                gregorian(1893, 12, 10),
                gregorian(1894, 1, 9),
                gregorian(1894, 2, 7),
                gregorian(1894, 3, 9),
                gregorian(1894, 4, 7),
                gregorian(1894, 5, 7),
                gregorian(1894, 6, 5),
                gregorian(1894, 7, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1312,
            [
                gregorian(1894, 7, 4),
                gregorian(1894, 8, 3),
                gregorian(1894, 9, 1),
                gregorian(1894, 10, 1),
                gregorian(1894, 10, 30),
                gregorian(1894, 11, 29),
                gregorian(1894, 12, 29),
                gregorian(1895, 1, 27),
                gregorian(1895, 2, 26),
                gregorian(1895, 3, 28),
                gregorian(1895, 4, 26),
                gregorian(1895, 5, 26),
                gregorian(1895, 6, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1313,
            [
                gregorian(1895, 6, 24),
                gregorian(1895, 7, 23),
                gregorian(1895, 8, 22),
                gregorian(1895, 9, 20),
                gregorian(1895, 10, 20),
                gregorian(1895, 11, 18),
                gregorian(1895, 12, 18),
                gregorian(1896, 1, 16),
                gregorian(1896, 2, 15),
                gregorian(1896, 3, 16),
                gregorian(1896, 4, 15),
                gregorian(1896, 5, 14),
                gregorian(1896, 6, 12),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1314,
            [
                gregorian(1896, 6, 12),
                gregorian(1896, 7, 12),
                gregorian(1896, 8, 11),
                gregorian(1896, 9, 9),
                gregorian(1896, 10, 9),
                gregorian(1896, 11, 7),
                gregorian(1896, 12, 6),
                gregorian(1897, 1, 5),
                gregorian(1897, 2, 3),
                gregorian(1897, 3, 5),
                gregorian(1897, 4, 4),
                gregorian(1897, 5, 3),
                gregorian(1897, 6, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1315,
            [
                gregorian(1897, 6, 2),
                gregorian(1897, 7, 1),
                gregorian(1897, 7, 31),
                gregorian(1897, 8, 30),
                gregorian(1897, 9, 28),
                gregorian(1897, 10, 28),
                gregorian(1897, 11, 26),
                gregorian(1897, 12, 25),
                gregorian(1898, 1, 24),
                gregorian(1898, 2, 22),
                gregorian(1898, 3, 24),
                gregorian(1898, 4, 22),
                gregorian(1898, 5, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1316,
            [
                gregorian(1898, 5, 22),
                gregorian(1898, 6, 20),
                gregorian(1898, 7, 20),
                gregorian(1898, 8, 19),
                gregorian(1898, 9, 18),
                gregorian(1898, 10, 17),
                gregorian(1898, 11, 16),
                gregorian(1898, 12, 15),
                gregorian(1899, 1, 13),
                gregorian(1899, 2, 12),
                gregorian(1899, 3, 13),
                gregorian(1899, 4, 12),
                gregorian(1899, 5, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1317,
            [
                gregorian(1899, 5, 11),
                gregorian(1899, 6, 10),
                gregorian(1899, 7, 9),
                gregorian(1899, 8, 8),
                gregorian(1899, 9, 7),
                gregorian(1899, 10, 6),
                gregorian(1899, 11, 5),
                gregorian(1899, 12, 4),
                gregorian(1900, 1, 3),
                gregorian(1900, 2, 1),
                gregorian(1900, 3, 3),
                gregorian(1900, 4, 1),
                gregorian(1900, 4, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1318,
            [
                gregorian(1900, 4, 30),
                gregorian(1900, 5, 30),
                gregorian(1900, 6, 28),
                gregorian(1900, 7, 28),
                gregorian(1900, 8, 27),
                gregorian(1900, 9, 25),
                gregorian(1900, 10, 25),
                gregorian(1900, 11, 24),
                gregorian(1900, 12, 23),
                gregorian(1901, 1, 22),
                gregorian(1901, 2, 20),
                gregorian(1901, 3, 22),
                gregorian(1901, 4, 20),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1319,
            [
                gregorian(1901, 4, 20),
                gregorian(1901, 5, 19),
                gregorian(1901, 6, 18),
                gregorian(1901, 7, 17),
                gregorian(1901, 8, 16),
                gregorian(1901, 9, 15),
                gregorian(1901, 10, 14),
                gregorian(1901, 11, 13),
                gregorian(1901, 12, 12),
                gregorian(1902, 1, 11),
                gregorian(1902, 2, 10),
                gregorian(1902, 3, 11),
                gregorian(1902, 4, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1320,
            [
                gregorian(1902, 4, 10),
                gregorian(1902, 5, 9),
                gregorian(1902, 6, 8),
                gregorian(1902, 7, 7),
                gregorian(1902, 8, 5),
                gregorian(1902, 9, 4),
                gregorian(1902, 10, 3),
                gregorian(1902, 11, 2),
                gregorian(1902, 12, 1),
                gregorian(1902, 12, 31),
                gregorian(1903, 1, 30),
                gregorian(1903, 3, 1),
                gregorian(1903, 3, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1321,
            [
                gregorian(1903, 3, 30),
                gregorian(1903, 4, 29),
                gregorian(1903, 5, 28),
                gregorian(1903, 6, 27),
                gregorian(1903, 7, 26),
                gregorian(1903, 8, 24),
                gregorian(1903, 9, 23),
                gregorian(1903, 10, 22),
                gregorian(1903, 11, 20),
                gregorian(1903, 12, 20),
                gregorian(1904, 1, 19),
                gregorian(1904, 2, 18),
                gregorian(1904, 3, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1322,
            [
                gregorian(1904, 3, 19),
                gregorian(1904, 4, 17),
                gregorian(1904, 5, 17),
                gregorian(1904, 6, 15),
                gregorian(1904, 7, 15),
                gregorian(1904, 8, 13),
                gregorian(1904, 9, 11),
                gregorian(1904, 10, 10),
                gregorian(1904, 11, 9),
                gregorian(1904, 12, 8),
                gregorian(1905, 1, 7),
                gregorian(1905, 2, 6),
                gregorian(1905, 3, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1323,
            [
                gregorian(1905, 3, 8),
                gregorian(1905, 4, 6),
                gregorian(1905, 5, 6),
                gregorian(1905, 6, 5),
                gregorian(1905, 7, 4),
                gregorian(1905, 8, 3),
                gregorian(1905, 9, 1),
                gregorian(1905, 9, 30),
                gregorian(1905, 10, 29),
                gregorian(1905, 11, 28),
                gregorian(1905, 12, 27),
                gregorian(1906, 1, 26),
                gregorian(1906, 2, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1324,
            [
                gregorian(1906, 2, 25),
                gregorian(1906, 3, 26),
                gregorian(1906, 4, 25),
                gregorian(1906, 5, 25),
                gregorian(1906, 6, 23),
                gregorian(1906, 7, 23),
                gregorian(1906, 8, 21),
                gregorian(1906, 9, 20),
                gregorian(1906, 10, 19),
                gregorian(1906, 11, 17),
                gregorian(1906, 12, 17),
                gregorian(1907, 1, 15),
                gregorian(1907, 2, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1325,
            [
                gregorian(1907, 2, 14),
                gregorian(1907, 3, 16),
                gregorian(1907, 4, 14),
                gregorian(1907, 5, 14),
                gregorian(1907, 6, 12),
                gregorian(1907, 7, 12),
                gregorian(1907, 8, 11),
                gregorian(1907, 9, 9),
                gregorian(1907, 10, 9),
                gregorian(1907, 11, 7),
                gregorian(1907, 12, 7),
                gregorian(1908, 1, 5),
                gregorian(1908, 2, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1326,
            [
                gregorian(1908, 2, 4),
                gregorian(1908, 3, 4),
                gregorian(1908, 4, 2),
                gregorian(1908, 5, 2),
                gregorian(1908, 5, 31),
                gregorian(1908, 6, 30),
                gregorian(1908, 7, 30),
                gregorian(1908, 8, 28),
                gregorian(1908, 9, 27),
                gregorian(1908, 10, 26),
                gregorian(1908, 11, 25),
                gregorian(1908, 12, 25),
                gregorian(1909, 1, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1327,
            [
                gregorian(1909, 1, 23),
                gregorian(1909, 2, 22),
                gregorian(1909, 3, 23),
                gregorian(1909, 4, 21),
                gregorian(1909, 5, 21),
                gregorian(1909, 6, 19),
                gregorian(1909, 7, 19),
                gregorian(1909, 8, 17),
                gregorian(1909, 9, 16),
                gregorian(1909, 10, 16),
                gregorian(1909, 11, 14),
                gregorian(1909, 12, 14),
                gregorian(1910, 1, 13),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1328,
            [
                gregorian(1910, 1, 13),
                gregorian(1910, 2, 11),
                gregorian(1910, 3, 13),
                gregorian(1910, 4, 11),
                gregorian(1910, 5, 10),
                gregorian(1910, 6, 9),
                gregorian(1910, 7, 8),
                gregorian(1910, 8, 6),
                gregorian(1910, 9, 5),
                gregorian(1910, 10, 5),
                gregorian(1910, 11, 4),
                gregorian(1910, 12, 3),
                gregorian(1911, 1, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1329,
            [
                gregorian(1911, 1, 2),
                gregorian(1911, 2, 1),
                gregorian(1911, 3, 2),
                gregorian(1911, 4, 1),
                gregorian(1911, 4, 30),
                gregorian(1911, 5, 29),
                gregorian(1911, 6, 28),
                gregorian(1911, 7, 27),
                gregorian(1911, 8, 25),
                gregorian(1911, 9, 24),
                gregorian(1911, 10, 24),
                gregorian(1911, 11, 22),
                gregorian(1911, 12, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1330,
            [
                gregorian(1911, 12, 22),
                gregorian(1912, 1, 21),
                gregorian(1912, 2, 20),
                gregorian(1912, 3, 20),
                gregorian(1912, 4, 19),
                gregorian(1912, 5, 18),
                gregorian(1912, 6, 16),
                gregorian(1912, 7, 16),
                gregorian(1912, 8, 14),
                gregorian(1912, 9, 12),
                gregorian(1912, 10, 12),
                gregorian(1912, 11, 11),
                gregorian(1912, 12, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1331,
            [
                gregorian(1912, 12, 10),
                gregorian(1913, 1, 9),
                gregorian(1913, 2, 8),
                gregorian(1913, 3, 9),
                gregorian(1913, 4, 8),
                gregorian(1913, 5, 8),
                gregorian(1913, 6, 6),
                gregorian(1913, 7, 5),
                gregorian(1913, 8, 4),
                gregorian(1913, 9, 2),
                gregorian(1913, 10, 2),
                gregorian(1913, 10, 31),
                gregorian(1913, 11, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1332,
            [
                gregorian(1913, 11, 30),
                gregorian(1913, 12, 29),
                gregorian(1914, 1, 28),
                gregorian(1914, 2, 26),
                gregorian(1914, 3, 28),
                gregorian(1914, 4, 27),
                gregorian(1914, 5, 26),
                gregorian(1914, 6, 25),
                gregorian(1914, 7, 24),
                gregorian(1914, 8, 23),
                gregorian(1914, 9, 22),
                gregorian(1914, 10, 21),
                gregorian(1914, 11, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1333,
            [
                gregorian(1914, 11, 19),
                gregorian(1914, 12, 19),
                gregorian(1915, 1, 17),
                gregorian(1915, 2, 15),
                gregorian(1915, 3, 17),
                gregorian(1915, 4, 16),
                gregorian(1915, 5, 15),
                gregorian(1915, 6, 14),
                gregorian(1915, 7, 14),
                gregorian(1915, 8, 12),
                gregorian(1915, 9, 11),
                gregorian(1915, 10, 11),
                gregorian(1915, 11, 9),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1334,
            [
                gregorian(1915, 11, 9),
                gregorian(1915, 12, 8),
                gregorian(1916, 1, 6),
                gregorian(1916, 2, 5),
                gregorian(1916, 3, 5),
                gregorian(1916, 4, 4),
                gregorian(1916, 5, 3),
                gregorian(1916, 6, 2),
                gregorian(1916, 7, 2),
                gregorian(1916, 8, 1),
                gregorian(1916, 8, 30),
                gregorian(1916, 9, 29),
                gregorian(1916, 10, 28),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1335,
            [
                gregorian(1916, 10, 28),
                gregorian(1916, 11, 27),
                gregorian(1916, 12, 26),
                gregorian(1917, 1, 25),
                gregorian(1917, 2, 23),
                gregorian(1917, 3, 24),
                gregorian(1917, 4, 23),
                gregorian(1917, 5, 22),
                gregorian(1917, 6, 21),
                gregorian(1917, 7, 21),
                gregorian(1917, 8, 19),
                gregorian(1917, 9, 18),
                gregorian(1917, 10, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1336,
            [
                gregorian(1917, 10, 18),
                gregorian(1917, 11, 16),
                gregorian(1917, 12, 16),
                gregorian(1918, 1, 14),
                gregorian(1918, 2, 13),
                gregorian(1918, 3, 14),
                gregorian(1918, 4, 12),
                gregorian(1918, 5, 12),
                gregorian(1918, 6, 10),
                gregorian(1918, 7, 10),
                gregorian(1918, 8, 8),
                gregorian(1918, 9, 7),
                gregorian(1918, 10, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1337,
            [
                gregorian(1918, 10, 7),
                gregorian(1918, 11, 6),
                gregorian(1918, 12, 5),
                gregorian(1919, 1, 4),
                gregorian(1919, 2, 2),
                gregorian(1919, 3, 4),
                gregorian(1919, 4, 2),
                gregorian(1919, 5, 1),
                gregorian(1919, 5, 31),
                gregorian(1919, 6, 29),
                gregorian(1919, 7, 29),
                gregorian(1919, 8, 27),
                gregorian(1919, 9, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1338,
            [
                gregorian(1919, 9, 26),
                gregorian(1919, 10, 25),
                gregorian(1919, 11, 24),
                gregorian(1919, 12, 24),
                gregorian(1920, 1, 22),
                gregorian(1920, 2, 21),
                gregorian(1920, 3, 22),
                gregorian(1920, 4, 20),
                gregorian(1920, 5, 19),
                gregorian(1920, 6, 18),
                gregorian(1920, 7, 17),
                gregorian(1920, 8, 16),
                gregorian(1920, 9, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1339,
            [
                gregorian(1920, 9, 14),
                gregorian(1920, 10, 14),
                gregorian(1920, 11, 12),
                gregorian(1920, 12, 12),
                gregorian(1921, 1, 10),
                gregorian(1921, 2, 9),
                gregorian(1921, 3, 11),
                gregorian(1921, 4, 10),
                gregorian(1921, 5, 9),
                gregorian(1921, 6, 8),
                gregorian(1921, 7, 7),
                gregorian(1921, 8, 5),
                gregorian(1921, 9, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1340,
            [
                gregorian(1921, 9, 4),
                gregorian(1921, 10, 3),
                gregorian(1921, 11, 1),
                gregorian(1921, 12, 1),
                gregorian(1921, 12, 30),
                gregorian(1922, 1, 29),
                gregorian(1922, 2, 28),
                gregorian(1922, 3, 30),
                gregorian(1922, 4, 29),
                gregorian(1922, 5, 28),
                gregorian(1922, 6, 27),
                gregorian(1922, 7, 26),
                gregorian(1922, 8, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1341,
            [
                gregorian(1922, 8, 24),
                gregorian(1922, 9, 23),
                gregorian(1922, 10, 22),
                gregorian(1922, 11, 20),
                gregorian(1922, 12, 20),
                gregorian(1923, 1, 18),
                gregorian(1923, 2, 17),
                gregorian(1923, 3, 19),
                gregorian(1923, 4, 18),
                gregorian(1923, 5, 17),
                gregorian(1923, 6, 16),
                gregorian(1923, 7, 16),
                gregorian(1923, 8, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1342,
            [
                gregorian(1923, 8, 14),
                gregorian(1923, 9, 12),
                gregorian(1923, 10, 11),
                gregorian(1923, 11, 10),
                gregorian(1923, 12, 9),
                gregorian(1924, 1, 8),
                gregorian(1924, 2, 6),
                gregorian(1924, 3, 7),
                gregorian(1924, 4, 6),
                gregorian(1924, 5, 5),
                gregorian(1924, 6, 4),
                gregorian(1924, 7, 4),
                gregorian(1924, 8, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1343,
            [
                gregorian(1924, 8, 2),
                gregorian(1924, 9, 1),
                gregorian(1924, 9, 30),
                gregorian(1924, 10, 29),
                gregorian(1924, 11, 28),
                gregorian(1924, 12, 27),
                gregorian(1925, 1, 26),
                gregorian(1925, 2, 24),
                gregorian(1925, 3, 26),
                gregorian(1925, 4, 24),
                gregorian(1925, 5, 24),
                gregorian(1925, 6, 23),
                gregorian(1925, 7, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1344,
            [
                gregorian(1925, 7, 22),
                gregorian(1925, 8, 21),
                gregorian(1925, 9, 19),
                gregorian(1925, 10, 19),
                gregorian(1925, 11, 17),
                gregorian(1925, 12, 17),
                gregorian(1926, 1, 16),
                gregorian(1926, 2, 14),
                gregorian(1926, 3, 15),
                gregorian(1926, 4, 14),
                gregorian(1926, 5, 13),
                gregorian(1926, 6, 12),
                gregorian(1926, 7, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1345,
            [
                gregorian(1926, 7, 11),
                gregorian(1926, 8, 10),
                gregorian(1926, 9, 8),
                gregorian(1926, 10, 8),
                gregorian(1926, 11, 7),
                gregorian(1926, 12, 7),
                gregorian(1927, 1, 5),
                gregorian(1927, 2, 4),
                gregorian(1927, 3, 5),
                gregorian(1927, 4, 3),
                gregorian(1927, 5, 3),
                gregorian(1927, 6, 1),
                gregorian(1927, 6, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1346,
            [
                gregorian(1927, 6, 30),
                gregorian(1927, 7, 30),
                gregorian(1927, 8, 28),
                gregorian(1927, 9, 27),
                gregorian(1927, 10, 27),
                gregorian(1927, 11, 26),
                gregorian(1927, 12, 26),
                gregorian(1928, 1, 24),
                gregorian(1928, 2, 23),
                gregorian(1928, 3, 23),
                gregorian(1928, 4, 21),
                gregorian(1928, 5, 21),
                gregorian(1928, 6, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1347,
            [
                gregorian(1928, 6, 19),
                gregorian(1928, 7, 18),
                gregorian(1928, 8, 17),
                gregorian(1928, 9, 15),
                gregorian(1928, 10, 15),
                gregorian(1928, 11, 14),
                gregorian(1928, 12, 14),
                gregorian(1929, 1, 12),
                gregorian(1929, 2, 11),
                gregorian(1929, 3, 13),
                gregorian(1929, 4, 11),
                gregorian(1929, 5, 10),
                gregorian(1929, 6, 9),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1348,
            [
                gregorian(1929, 6, 9),
                gregorian(1929, 7, 8),
                gregorian(1929, 8, 6),
                gregorian(1929, 9, 5),
                gregorian(1929, 10, 4),
                gregorian(1929, 11, 3),
                gregorian(1929, 12, 3),
                gregorian(1930, 1, 1),
                gregorian(1930, 1, 31),
                gregorian(1930, 3, 2),
                gregorian(1930, 4, 1),
                gregorian(1930, 4, 30),
                gregorian(1930, 5, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1349,
            [
                gregorian(1930, 5, 29),
                gregorian(1930, 6, 28),
                gregorian(1930, 7, 27),
                gregorian(1930, 8, 25),
                gregorian(1930, 9, 24),
                gregorian(1930, 10, 23),
                gregorian(1930, 11, 22),
                gregorian(1930, 12, 22),
                gregorian(1931, 1, 20),
                gregorian(1931, 2, 19),
                gregorian(1931, 3, 21),
                gregorian(1931, 4, 19),
                gregorian(1931, 5, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1350,
            [
                gregorian(1931, 5, 19),
                gregorian(1931, 6, 17),
                gregorian(1931, 7, 17),
                gregorian(1931, 8, 15),
                gregorian(1931, 9, 14),
                gregorian(1931, 10, 13),
                gregorian(1931, 11, 12),
                gregorian(1931, 12, 11),
                gregorian(1932, 1, 9),
                gregorian(1932, 2, 8),
                gregorian(1932, 3, 9),
                gregorian(1932, 4, 7),
                gregorian(1932, 5, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1351,
            [
                gregorian(1932, 5, 7),
                gregorian(1932, 6, 6),
                gregorian(1932, 7, 5),
                gregorian(1932, 8, 4),
                gregorian(1932, 9, 2),
                gregorian(1932, 10, 2),
                gregorian(1932, 10, 31),
                gregorian(1932, 11, 30),
                gregorian(1932, 12, 29),
                gregorian(1933, 1, 27),
                gregorian(1933, 2, 26),
                gregorian(1933, 3, 27),
                gregorian(1933, 4, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1352,
            [
                gregorian(1933, 4, 26),
                gregorian(1933, 5, 26),
                gregorian(1933, 6, 24),
                gregorian(1933, 7, 24),
                gregorian(1933, 8, 23),
                gregorian(1933, 9, 21),
                gregorian(1933, 10, 21),
                gregorian(1933, 11, 19),
                gregorian(1933, 12, 19),
                gregorian(1934, 1, 17),
                gregorian(1934, 2, 15),
                gregorian(1934, 3, 17),
                gregorian(1934, 4, 15),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1353,
            [
                gregorian(1934, 4, 15),
                gregorian(1934, 5, 15),
                gregorian(1934, 6, 13),
                gregorian(1934, 7, 13),
                gregorian(1934, 8, 12),
                gregorian(1934, 9, 11),
                gregorian(1934, 10, 10),
                gregorian(1934, 11, 9),
                gregorian(1934, 12, 8),
                gregorian(1935, 1, 6),
                gregorian(1935, 2, 5),
                gregorian(1935, 3, 6),
                gregorian(1935, 4, 5),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1354,
            [
                gregorian(1935, 4, 5),
                gregorian(1935, 5, 4),
                gregorian(1935, 6, 3),
                gregorian(1935, 7, 2),
                gregorian(1935, 8, 1),
                gregorian(1935, 8, 31),
                gregorian(1935, 9, 29),
                gregorian(1935, 10, 29),
                gregorian(1935, 11, 28),
                gregorian(1935, 12, 27),
                gregorian(1936, 1, 26),
                gregorian(1936, 2, 24),
                gregorian(1936, 3, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1355,
            [
                gregorian(1936, 3, 24),
                gregorian(1936, 4, 23),
                gregorian(1936, 5, 22),
                gregorian(1936, 6, 20),
                gregorian(1936, 7, 20),
                gregorian(1936, 8, 19),
                gregorian(1936, 9, 17),
                gregorian(1936, 10, 17),
                gregorian(1936, 11, 16),
                gregorian(1936, 12, 15),
                gregorian(1937, 1, 14),
                gregorian(1937, 2, 13),
                gregorian(1937, 3, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1356,
            [
                gregorian(1937, 3, 14),
                gregorian(1937, 4, 12),
                gregorian(1937, 5, 12),
                gregorian(1937, 6, 10),
                gregorian(1937, 7, 10),
                gregorian(1937, 8, 8),
                gregorian(1937, 9, 7),
                gregorian(1937, 10, 6),
                gregorian(1937, 11, 5),
                gregorian(1937, 12, 4),
                gregorian(1938, 1, 3),
                gregorian(1938, 2, 2),
                gregorian(1938, 3, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1357,
            [
                gregorian(1938, 3, 4),
                gregorian(1938, 4, 2),
                gregorian(1938, 5, 1),
                gregorian(1938, 5, 31),
                gregorian(1938, 6, 29),
                gregorian(1938, 7, 29),
                gregorian(1938, 8, 27),
                gregorian(1938, 9, 25),
                gregorian(1938, 10, 25),
                gregorian(1938, 11, 23),
                gregorian(1938, 12, 23),
                gregorian(1939, 1, 22),
                gregorian(1939, 2, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1358,
            [
                gregorian(1939, 2, 21),
                gregorian(1939, 3, 22),
                gregorian(1939, 4, 21),
                gregorian(1939, 5, 20),
                gregorian(1939, 6, 19),
                gregorian(1939, 7, 18),
                gregorian(1939, 8, 17),
                gregorian(1939, 9, 15),
                gregorian(1939, 10, 14),
                gregorian(1939, 11, 13),
                gregorian(1939, 12, 12),
                gregorian(1940, 1, 11),
                gregorian(1940, 2, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1359,
            [
                gregorian(1940, 2, 10),
                gregorian(1940, 3, 10),
                gregorian(1940, 4, 9),
                gregorian(1940, 5, 9),
                gregorian(1940, 6, 7),
                gregorian(1940, 7, 7),
                gregorian(1940, 8, 5),
                gregorian(1940, 9, 4),
                gregorian(1940, 10, 3),
                gregorian(1940, 11, 1),
                gregorian(1940, 11, 30),
                gregorian(1940, 12, 30),
                gregorian(1941, 1, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1360,
            [
                gregorian(1941, 1, 29),
                gregorian(1941, 2, 27),
                gregorian(1941, 3, 29),
                gregorian(1941, 4, 28),
                gregorian(1941, 5, 28),
                gregorian(1941, 6, 26),
                gregorian(1941, 7, 26),
                gregorian(1941, 8, 24),
                gregorian(1941, 9, 23),
                gregorian(1941, 10, 22),
                gregorian(1941, 11, 20),
                gregorian(1941, 12, 20),
                gregorian(1942, 1, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1361,
            [
                gregorian(1942, 1, 18),
                gregorian(1942, 2, 17),
                gregorian(1942, 3, 18),
                gregorian(1942, 4, 17),
                gregorian(1942, 5, 17),
                gregorian(1942, 6, 15),
                gregorian(1942, 7, 15),
                gregorian(1942, 8, 14),
                gregorian(1942, 9, 12),
                gregorian(1942, 10, 11),
                gregorian(1942, 11, 10),
                gregorian(1942, 12, 9),
                gregorian(1943, 1, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1362,
            [
                gregorian(1943, 1, 8),
                gregorian(1943, 2, 6),
                gregorian(1943, 3, 8),
                gregorian(1943, 4, 6),
                gregorian(1943, 5, 6),
                gregorian(1943, 6, 4),
                gregorian(1943, 7, 4),
                gregorian(1943, 8, 3),
                gregorian(1943, 9, 1),
                gregorian(1943, 10, 1),
                gregorian(1943, 10, 30),
                gregorian(1943, 11, 29),
                gregorian(1943, 12, 28),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1363,
            [
                gregorian(1943, 12, 28),
                gregorian(1944, 1, 27),
                gregorian(1944, 2, 25),
                gregorian(1944, 3, 26),
                gregorian(1944, 4, 24),
                gregorian(1944, 5, 24),
                gregorian(1944, 6, 22),
                gregorian(1944, 7, 22),
                gregorian(1944, 8, 20),
                gregorian(1944, 9, 19),
                gregorian(1944, 10, 18),
                gregorian(1944, 11, 17),
                gregorian(1944, 12, 17),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1364,
            [
                gregorian(1944, 12, 17),
                gregorian(1945, 1, 15),
                gregorian(1945, 2, 14),
                gregorian(1945, 3, 15),
                gregorian(1945, 4, 14),
                gregorian(1945, 5, 13),
                gregorian(1945, 6, 11),
                gregorian(1945, 7, 11),
                gregorian(1945, 8, 9),
                gregorian(1945, 9, 8),
                gregorian(1945, 10, 7),
                gregorian(1945, 11, 6),
                gregorian(1945, 12, 6),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1365,
            [
                gregorian(1945, 12, 6),
                gregorian(1946, 1, 5),
                gregorian(1946, 2, 4),
                gregorian(1946, 3, 5),
                gregorian(1946, 4, 3),
                gregorian(1946, 5, 3),
                gregorian(1946, 6, 1),
                gregorian(1946, 6, 30),
                gregorian(1946, 7, 30),
                gregorian(1946, 8, 28),
                gregorian(1946, 9, 27),
                gregorian(1946, 10, 26),
                gregorian(1946, 11, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1366,
            [
                gregorian(1946, 11, 25),
                gregorian(1946, 12, 25),
                gregorian(1947, 1, 24),
                gregorian(1947, 2, 22),
                gregorian(1947, 3, 24),
                gregorian(1947, 4, 22),
                gregorian(1947, 5, 22),
                gregorian(1947, 6, 20),
                gregorian(1947, 7, 19),
                gregorian(1947, 8, 18),
                gregorian(1947, 9, 16),
                gregorian(1947, 10, 16),
                gregorian(1947, 11, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1367,
            [
                gregorian(1947, 11, 14),
                gregorian(1947, 12, 14),
                gregorian(1948, 1, 13),
                gregorian(1948, 2, 11),
                gregorian(1948, 3, 12),
                gregorian(1948, 4, 11),
                gregorian(1948, 5, 10),
                gregorian(1948, 6, 9),
                gregorian(1948, 7, 8),
                gregorian(1948, 8, 6),
                gregorian(1948, 9, 5),
                gregorian(1948, 10, 4),
                gregorian(1948, 11, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1368,
            [
                gregorian(1948, 11, 3),
                gregorian(1948, 12, 2),
                gregorian(1949, 1, 1),
                gregorian(1949, 1, 30),
                gregorian(1949, 3, 1),
                gregorian(1949, 3, 31),
                gregorian(1949, 4, 30),
                gregorian(1949, 5, 29),
                gregorian(1949, 6, 27),
                gregorian(1949, 7, 27),
                gregorian(1949, 8, 25),
                gregorian(1949, 9, 24),
                gregorian(1949, 10, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1369,
            [
                gregorian(1949, 10, 23),
                gregorian(1949, 11, 22),
                gregorian(1949, 12, 21),
                gregorian(1950, 1, 20),
                gregorian(1950, 2, 18),
                gregorian(1950, 3, 20),
                gregorian(1950, 4, 19),
                gregorian(1950, 5, 18),
                gregorian(1950, 6, 17),
                gregorian(1950, 7, 16),
                gregorian(1950, 8, 15),
                gregorian(1950, 9, 14),
                gregorian(1950, 10, 13),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1370,
            [
                gregorian(1950, 10, 13),
                gregorian(1950, 11, 12),
                gregorian(1950, 12, 11),
                gregorian(1951, 1, 9),
                gregorian(1951, 2, 8),
                gregorian(1951, 3, 9),
                gregorian(1951, 4, 8),
                gregorian(1951, 5, 7),
                gregorian(1951, 6, 6),
                gregorian(1951, 7, 5),
                gregorian(1951, 8, 4),
                gregorian(1951, 9, 3),
                gregorian(1951, 10, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1371,
            [
                gregorian(1951, 10, 3),
                gregorian(1951, 11, 1),
                gregorian(1951, 12, 1),
                gregorian(1951, 12, 30),
                gregorian(1952, 1, 28),
                gregorian(1952, 2, 27),
                gregorian(1952, 3, 27),
                gregorian(1952, 4, 26),
                gregorian(1952, 5, 25),
                gregorian(1952, 6, 24),
                gregorian(1952, 7, 23),
                gregorian(1952, 8, 22),
                gregorian(1952, 9, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1372,
            [
                gregorian(1952, 9, 21),
                gregorian(1952, 10, 21),
                gregorian(1952, 11, 19),
                gregorian(1952, 12, 18),
                gregorian(1953, 1, 17),
                gregorian(1953, 2, 15),
                gregorian(1953, 3, 17),
                gregorian(1953, 4, 15),
                gregorian(1953, 5, 14),
                gregorian(1953, 6, 13),
                gregorian(1953, 7, 12),
                gregorian(1953, 8, 11),
                gregorian(1953, 9, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1373,
            [
                gregorian(1953, 9, 10),
                gregorian(1953, 10, 10),
                gregorian(1953, 11, 8),
                gregorian(1953, 12, 8),
                gregorian(1954, 1, 6),
                gregorian(1954, 2, 5),
                gregorian(1954, 3, 6),
                gregorian(1954, 4, 5),
                gregorian(1954, 5, 4),
                gregorian(1954, 6, 2),
                gregorian(1954, 7, 2),
                gregorian(1954, 7, 31),
                gregorian(1954, 8, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1374,
            [
                gregorian(1954, 8, 30),
                gregorian(1954, 9, 29),
                gregorian(1954, 10, 28),
                gregorian(1954, 11, 27),
                gregorian(1954, 12, 27),
                gregorian(1955, 1, 25),
                gregorian(1955, 2, 24),
                gregorian(1955, 3, 25),
                gregorian(1955, 4, 24),
                gregorian(1955, 5, 23),
                gregorian(1955, 6, 21),
                gregorian(1955, 7, 21),
                gregorian(1955, 8, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1375,
            [
                gregorian(1955, 8, 19),
                gregorian(1955, 9, 18),
                gregorian(1955, 10, 17),
                gregorian(1955, 11, 16),
                gregorian(1955, 12, 16),
                gregorian(1956, 1, 14),
                gregorian(1956, 2, 13),
                gregorian(1956, 3, 14),
                gregorian(1956, 4, 12),
                gregorian(1956, 5, 12),
                gregorian(1956, 6, 10),
                gregorian(1956, 7, 10),
                gregorian(1956, 8, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1376,
            [
                gregorian(1956, 8, 8),
                gregorian(1956, 9, 6),
                gregorian(1956, 10, 6),
                gregorian(1956, 11, 4),
                gregorian(1956, 12, 4),
                gregorian(1957, 1, 2),
                gregorian(1957, 2, 1),
                gregorian(1957, 3, 3),
                gregorian(1957, 4, 2),
                gregorian(1957, 5, 1),
                gregorian(1957, 5, 31),
                gregorian(1957, 6, 29),
                gregorian(1957, 7, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1377,
            [
                gregorian(1957, 7, 29),
                gregorian(1957, 8, 27),
                gregorian(1957, 9, 25),
                gregorian(1957, 10, 25),
                gregorian(1957, 11, 23),
                gregorian(1957, 12, 22),
                gregorian(1958, 1, 21),
                gregorian(1958, 2, 20),
                gregorian(1958, 3, 22),
                gregorian(1958, 4, 20),
                gregorian(1958, 5, 20),
                gregorian(1958, 6, 19),
                gregorian(1958, 7, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1378,
            [
                gregorian(1958, 7, 18),
                gregorian(1958, 8, 17),
                gregorian(1958, 9, 15),
                gregorian(1958, 10, 14),
                gregorian(1958, 11, 12),
                gregorian(1958, 12, 12),
                gregorian(1959, 1, 10),
                gregorian(1959, 2, 9),
                gregorian(1959, 3, 11),
                gregorian(1959, 4, 9),
                gregorian(1959, 5, 9),
                gregorian(1959, 6, 8),
                gregorian(1959, 7, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1379,
            [
                gregorian(1959, 7, 8),
                gregorian(1959, 8, 6),
                gregorian(1959, 9, 5),
                gregorian(1959, 10, 4),
                gregorian(1959, 11, 2),
                gregorian(1959, 12, 1),
                gregorian(1959, 12, 31),
                gregorian(1960, 1, 29),
                gregorian(1960, 2, 28),
                gregorian(1960, 3, 29),
                gregorian(1960, 4, 27),
                gregorian(1960, 5, 27),
                gregorian(1960, 6, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1380,
            [
                gregorian(1960, 6, 26),
                gregorian(1960, 7, 25),
                gregorian(1960, 8, 24),
                gregorian(1960, 9, 22),
                gregorian(1960, 10, 22),
                gregorian(1960, 11, 20),
                gregorian(1960, 12, 20),
                gregorian(1961, 1, 18),
                gregorian(1961, 2, 17),
                gregorian(1961, 3, 18),
                gregorian(1961, 4, 17),
                gregorian(1961, 5, 16),
                gregorian(1961, 6, 15),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1381,
            [
                gregorian(1961, 6, 15),
                gregorian(1961, 7, 14),
                gregorian(1961, 8, 13),
                gregorian(1961, 9, 11),
                gregorian(1961, 10, 11),
                gregorian(1961, 11, 10),
                gregorian(1961, 12, 9),
                gregorian(1962, 1, 8),
                gregorian(1962, 2, 6),
                gregorian(1962, 3, 8),
                gregorian(1962, 4, 6),
                gregorian(1962, 5, 5),
                gregorian(1962, 6, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1382,
            [
                gregorian(1962, 6, 4),
                gregorian(1962, 7, 3),
                gregorian(1962, 8, 2),
                gregorian(1962, 8, 31),
                gregorian(1962, 9, 30),
                gregorian(1962, 10, 30),
                gregorian(1962, 11, 28),
                gregorian(1962, 12, 28),
                gregorian(1963, 1, 27),
                gregorian(1963, 2, 25),
                gregorian(1963, 3, 27),
                gregorian(1963, 4, 25),
                gregorian(1963, 5, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1383,
            [
                gregorian(1963, 5, 24),
                gregorian(1963, 6, 23),
                gregorian(1963, 7, 22),
                gregorian(1963, 8, 20),
                gregorian(1963, 9, 19),
                gregorian(1963, 10, 19),
                gregorian(1963, 11, 18),
                gregorian(1963, 12, 17),
                gregorian(1964, 1, 16),
                gregorian(1964, 2, 15),
                gregorian(1964, 3, 15),
                gregorian(1964, 4, 14),
                gregorian(1964, 5, 13),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1384,
            [
                gregorian(1964, 5, 13),
                gregorian(1964, 6, 11),
                gregorian(1964, 7, 11),
                gregorian(1964, 8, 9),
                gregorian(1964, 9, 7),
                gregorian(1964, 10, 7),
                gregorian(1964, 11, 6),
                gregorian(1964, 12, 5),
                gregorian(1965, 1, 4),
                gregorian(1965, 2, 3),
                gregorian(1965, 3, 5),
                gregorian(1965, 4, 3),
                gregorian(1965, 5, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1385,
            [
                gregorian(1965, 5, 3),
                gregorian(1965, 6, 1),
                gregorian(1965, 6, 30),
                gregorian(1965, 7, 30),
                gregorian(1965, 8, 28),
                gregorian(1965, 9, 26),
                gregorian(1965, 10, 26),
                gregorian(1965, 11, 25),
                gregorian(1965, 12, 24),
                gregorian(1966, 1, 23),
                gregorian(1966, 2, 22),
                gregorian(1966, 3, 24),
                gregorian(1966, 4, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1386,
            [
                gregorian(1966, 4, 22),
                gregorian(1966, 5, 22),
                gregorian(1966, 6, 20),
                gregorian(1966, 7, 19),
                gregorian(1966, 8, 18),
                gregorian(1966, 9, 16),
                gregorian(1966, 10, 15),
                gregorian(1966, 11, 14),
                gregorian(1966, 12, 14),
                gregorian(1967, 1, 12),
                gregorian(1967, 2, 11),
                gregorian(1967, 3, 13),
                gregorian(1967, 4, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1387,
            [
                gregorian(1967, 4, 11),
                gregorian(1967, 5, 11),
                gregorian(1967, 6, 9),
                gregorian(1967, 7, 9),
                gregorian(1967, 8, 7),
                gregorian(1967, 9, 6),
                gregorian(1967, 10, 5),
                gregorian(1967, 11, 4),
                gregorian(1967, 12, 3),
                gregorian(1968, 1, 2),
                gregorian(1968, 1, 31),
                gregorian(1968, 3, 1),
                gregorian(1968, 3, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1388,
            [
                gregorian(1968, 3, 30),
                gregorian(1968, 4, 29),
                gregorian(1968, 5, 29),
                gregorian(1968, 6, 27),
                gregorian(1968, 7, 27),
                gregorian(1968, 8, 25),
                gregorian(1968, 9, 24),
                gregorian(1968, 10, 23),
                gregorian(1968, 11, 22),
                gregorian(1968, 12, 21),
                gregorian(1969, 1, 20),
                gregorian(1969, 2, 18),
                gregorian(1969, 3, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1389,
            [
                gregorian(1969, 3, 19),
                gregorian(1969, 4, 18),
                gregorian(1969, 5, 18),
                gregorian(1969, 6, 16),
                gregorian(1969, 7, 16),
                gregorian(1969, 8, 15),
                gregorian(1969, 9, 13),
                gregorian(1969, 10, 13),
                gregorian(1969, 11, 12),
                gregorian(1969, 12, 11),
                gregorian(1970, 1, 9),
                gregorian(1970, 2, 8),
                gregorian(1970, 3, 9),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1390,
            [
                gregorian(1970, 3, 9),
                gregorian(1970, 4, 7),
                gregorian(1970, 5, 7),
                gregorian(1970, 6, 5),
                gregorian(1970, 7, 5),
                gregorian(1970, 8, 4),
                gregorian(1970, 9, 3),
                gregorian(1970, 10, 2),
                gregorian(1970, 11, 1),
                gregorian(1970, 11, 30),
                gregorian(1970, 12, 30),
                gregorian(1971, 1, 28),
                gregorian(1971, 2, 27),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1391,
            [
                gregorian(1971, 2, 27),
                gregorian(1971, 3, 28),
                gregorian(1971, 4, 26),
                gregorian(1971, 5, 26),
                gregorian(1971, 6, 24),
                gregorian(1971, 7, 24),
                gregorian(1971, 8, 23),
                gregorian(1971, 9, 21),
                gregorian(1971, 10, 21),
                gregorian(1971, 11, 20),
                gregorian(1971, 12, 19),
                gregorian(1972, 1, 18),
                gregorian(1972, 2, 16),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1392,
            [
                gregorian(1972, 2, 16),
                gregorian(1972, 3, 17),
                gregorian(1972, 4, 15),
                gregorian(1972, 5, 14),
                gregorian(1972, 6, 13),
                gregorian(1972, 7, 12),
                gregorian(1972, 8, 11),
                gregorian(1972, 9, 9),
                gregorian(1972, 10, 9),
                gregorian(1972, 11, 8),
                gregorian(1972, 12, 7),
                gregorian(1973, 1, 6),
                gregorian(1973, 2, 5),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1393,
            [
                gregorian(1973, 2, 5),
                gregorian(1973, 3, 6),
                gregorian(1973, 4, 5),
                gregorian(1973, 5, 4),
                gregorian(1973, 6, 2),
                gregorian(1973, 7, 2),
                gregorian(1973, 7, 31),
                gregorian(1973, 8, 30),
                gregorian(1973, 9, 28),
                gregorian(1973, 10, 28),
                gregorian(1973, 11, 26),
                gregorian(1973, 12, 26),
                gregorian(1974, 1, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1394,
            [
                gregorian(1974, 1, 25),
                gregorian(1974, 2, 24),
                gregorian(1974, 3, 25),
                gregorian(1974, 4, 24),
                gregorian(1974, 5, 23),
                gregorian(1974, 6, 21),
                gregorian(1974, 7, 21),
                gregorian(1974, 8, 19),
                gregorian(1974, 9, 18),
                gregorian(1974, 10, 17),
                gregorian(1974, 11, 16),
                gregorian(1974, 12, 15),
                gregorian(1975, 1, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1395,
            [
                gregorian(1975, 1, 14),
                gregorian(1975, 2, 13),
                gregorian(1975, 3, 14),
                gregorian(1975, 4, 13),
                gregorian(1975, 5, 13),
                gregorian(1975, 6, 11),
                gregorian(1975, 7, 11),
                gregorian(1975, 8, 9),
                gregorian(1975, 9, 7),
                gregorian(1975, 10, 7),
                gregorian(1975, 11, 5),
                gregorian(1975, 12, 4),
                gregorian(1976, 1, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1396,
            [
                gregorian(1976, 1, 3),
                gregorian(1976, 2, 2),
                gregorian(1976, 3, 2),
                gregorian(1976, 4, 1),
                gregorian(1976, 5, 1),
                gregorian(1976, 5, 30),
                gregorian(1976, 6, 29),
                gregorian(1976, 7, 29),
                gregorian(1976, 8, 27),
                gregorian(1976, 9, 25),
                gregorian(1976, 10, 25),
                gregorian(1976, 11, 23),
                gregorian(1976, 12, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1397,
            [
                gregorian(1976, 12, 22),
                gregorian(1977, 1, 21),
                gregorian(1977, 2, 19),
                gregorian(1977, 3, 21),
                gregorian(1977, 4, 20),
                gregorian(1977, 5, 19),
                gregorian(1977, 6, 18),
                gregorian(1977, 7, 18),
                gregorian(1977, 8, 17),
                gregorian(1977, 9, 15),
                gregorian(1977, 10, 14),
                gregorian(1977, 11, 12),
                gregorian(1977, 12, 12),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1398,
            [
                gregorian(1977, 12, 12),
                gregorian(1978, 1, 10),
                gregorian(1978, 2, 9),
                gregorian(1978, 3, 10),
                gregorian(1978, 4, 9),
                gregorian(1978, 5, 9),
                gregorian(1978, 6, 7),
                gregorian(1978, 7, 7),
                gregorian(1978, 8, 6),
                gregorian(1978, 9, 4),
                gregorian(1978, 10, 4),
                gregorian(1978, 11, 2),
                gregorian(1978, 12, 1),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1399,
            [
                gregorian(1978, 12, 1),
                gregorian(1978, 12, 31),
                gregorian(1979, 1, 29),
                gregorian(1979, 2, 28),
                gregorian(1979, 3, 29),
                gregorian(1979, 4, 28),
                gregorian(1979, 5, 27),
                gregorian(1979, 6, 26),
                gregorian(1979, 7, 26),
                gregorian(1979, 8, 24),
                gregorian(1979, 9, 23),
                gregorian(1979, 10, 22),
                gregorian(1979, 11, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1400,
            [
                gregorian(1979, 11, 21),
                gregorian(1979, 12, 21),
                gregorian(1980, 1, 19),
                gregorian(1980, 2, 18),
                gregorian(1980, 3, 18),
                gregorian(1980, 4, 16),
                gregorian(1980, 5, 16),
                gregorian(1980, 6, 14),
                gregorian(1980, 7, 14),
                gregorian(1980, 8, 12),
                gregorian(1980, 9, 11),
                gregorian(1980, 10, 10),
                gregorian(1980, 11, 9),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1401,
            [
                gregorian(1980, 11, 9),
                gregorian(1980, 12, 9),
                gregorian(1981, 1, 8),
                gregorian(1981, 2, 6),
                gregorian(1981, 3, 8),
                gregorian(1981, 4, 6),
                gregorian(1981, 5, 5),
                gregorian(1981, 6, 4),
                gregorian(1981, 7, 3),
                gregorian(1981, 8, 1),
                gregorian(1981, 8, 31),
                gregorian(1981, 9, 29),
                gregorian(1981, 10, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1402,
            [
                gregorian(1981, 10, 29),
                gregorian(1981, 11, 28),
                gregorian(1981, 12, 28),
                gregorian(1982, 1, 27),
                gregorian(1982, 2, 25),
                gregorian(1982, 3, 27),
                gregorian(1982, 4, 25),
                gregorian(1982, 5, 24),
                gregorian(1982, 6, 23),
                gregorian(1982, 7, 22),
                gregorian(1982, 8, 20),
                gregorian(1982, 9, 19),
                gregorian(1982, 10, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1403,
            [
                gregorian(1982, 10, 18),
                gregorian(1982, 11, 17),
                gregorian(1982, 12, 17),
                gregorian(1983, 1, 16),
                gregorian(1983, 2, 14),
                gregorian(1983, 3, 16),
                gregorian(1983, 4, 15),
                gregorian(1983, 5, 14),
                gregorian(1983, 6, 12),
                gregorian(1983, 7, 12),
                gregorian(1983, 8, 10),
                gregorian(1983, 9, 8),
                gregorian(1983, 10, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1404,
            [
                gregorian(1983, 10, 8),
                gregorian(1983, 11, 6),
                gregorian(1983, 12, 6),
                gregorian(1984, 1, 5),
                gregorian(1984, 2, 3),
                gregorian(1984, 3, 4),
                gregorian(1984, 4, 3),
                gregorian(1984, 5, 2),
                gregorian(1984, 6, 1),
                gregorian(1984, 6, 30),
                gregorian(1984, 7, 30),
                gregorian(1984, 8, 28),
                gregorian(1984, 9, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1405,
            [
                gregorian(1984, 9, 26),
                gregorian(1984, 10, 26),
                gregorian(1984, 11, 24),
                gregorian(1984, 12, 24),
                gregorian(1985, 1, 22),
                gregorian(1985, 2, 21),
                gregorian(1985, 3, 23),
                gregorian(1985, 4, 22),
                gregorian(1985, 5, 21),
                gregorian(1985, 6, 20),
                gregorian(1985, 7, 19),
                gregorian(1985, 8, 17),
                gregorian(1985, 9, 16),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1406,
            [
                gregorian(1985, 9, 16),
                gregorian(1985, 10, 16),
                gregorian(1985, 11, 14),
                gregorian(1985, 12, 13),
                gregorian(1986, 1, 12),
                gregorian(1986, 2, 10),
                gregorian(1986, 3, 12),
                gregorian(1986, 4, 11),
                gregorian(1986, 5, 10),
                gregorian(1986, 6, 9),
                gregorian(1986, 7, 8),
                gregorian(1986, 8, 7),
                gregorian(1986, 9, 6),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1407,
            [
                gregorian(1986, 9, 6),
                gregorian(1986, 10, 5),
                gregorian(1986, 11, 4),
                gregorian(1986, 12, 3),
                gregorian(1987, 1, 1),
                gregorian(1987, 1, 31),
                gregorian(1987, 3, 1),
                gregorian(1987, 3, 31),
                gregorian(1987, 4, 29),
                gregorian(1987, 5, 29),
                gregorian(1987, 6, 27),
                gregorian(1987, 7, 27),
                gregorian(1987, 8, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1408,
            [
                gregorian(1987, 8, 26),
                gregorian(1987, 9, 25),
                gregorian(1987, 10, 24),
                gregorian(1987, 11, 23),
                gregorian(1987, 12, 22),
                gregorian(1988, 1, 21),
                gregorian(1988, 2, 19),
                gregorian(1988, 3, 19),
                gregorian(1988, 4, 18),
                gregorian(1988, 5, 17),
                gregorian(1988, 6, 15),
                gregorian(1988, 7, 15),
                gregorian(1988, 8, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1409,
            [
                gregorian(1988, 8, 14),
                gregorian(1988, 9, 13),
                gregorian(1988, 10, 13),
                gregorian(1988, 11, 11),
                gregorian(1988, 12, 11),
                gregorian(1989, 1, 9),
                gregorian(1989, 2, 8),
                gregorian(1989, 3, 9),
                gregorian(1989, 4, 7),
                gregorian(1989, 5, 7),
                gregorian(1989, 6, 5),
                gregorian(1989, 7, 4),
                gregorian(1989, 8, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1410,
            [
                gregorian(1989, 8, 3),
                gregorian(1989, 9, 2),
                gregorian(1989, 10, 2),
                gregorian(1989, 10, 31),
                gregorian(1989, 11, 30),
                gregorian(1989, 12, 30),
                gregorian(1990, 1, 28),
                gregorian(1990, 2, 27),
                gregorian(1990, 3, 28),
                gregorian(1990, 4, 26),
                gregorian(1990, 5, 26),
                gregorian(1990, 6, 24),
                gregorian(1990, 7, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1411,
            [
                gregorian(1990, 7, 23),
                gregorian(1990, 8, 22),
                gregorian(1990, 9, 21),
                gregorian(1990, 10, 20),
                gregorian(1990, 11, 19),
                gregorian(1990, 12, 19),
                gregorian(1991, 1, 17),
                gregorian(1991, 2, 16),
                gregorian(1991, 3, 18),
                gregorian(1991, 4, 16),
                gregorian(1991, 5, 15),
                gregorian(1991, 6, 14),
                gregorian(1991, 7, 13),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1412,
            [
                gregorian(1991, 7, 13),
                gregorian(1991, 8, 12),
                gregorian(1991, 9, 10),
                gregorian(1991, 10, 10),
                gregorian(1991, 11, 8),
                gregorian(1991, 12, 8),
                gregorian(1992, 1, 6),
                gregorian(1992, 2, 5),
                gregorian(1992, 3, 6),
                gregorian(1992, 4, 5),
                gregorian(1992, 5, 4),
                gregorian(1992, 6, 2),
                gregorian(1992, 7, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1413,
            [
                gregorian(1992, 7, 2),
                gregorian(1992, 7, 31),
                gregorian(1992, 8, 30),
                gregorian(1992, 9, 28),
                gregorian(1992, 10, 27),
                gregorian(1992, 11, 26),
                gregorian(1992, 12, 25),
                gregorian(1993, 1, 24),
                gregorian(1993, 2, 23),
                gregorian(1993, 3, 25),
                gregorian(1993, 4, 23),
                gregorian(1993, 5, 23),
                gregorian(1993, 6, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1414,
            [
                gregorian(1993, 6, 21),
                gregorian(1993, 7, 21),
                gregorian(1993, 8, 19),
                gregorian(1993, 9, 18),
                gregorian(1993, 10, 17),
                gregorian(1993, 11, 15),
                gregorian(1993, 12, 15),
                gregorian(1994, 1, 13),
                gregorian(1994, 2, 12),
                gregorian(1994, 3, 14),
                gregorian(1994, 4, 12),
                gregorian(1994, 5, 12),
                gregorian(1994, 6, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1415,
            [
                gregorian(1994, 6, 11),
                gregorian(1994, 7, 10),
                gregorian(1994, 8, 9),
                gregorian(1994, 9, 7),
                gregorian(1994, 10, 7),
                gregorian(1994, 11, 5),
                gregorian(1994, 12, 4),
                gregorian(1995, 1, 3),
                gregorian(1995, 2, 1),
                gregorian(1995, 3, 3),
                gregorian(1995, 4, 1),
                gregorian(1995, 5, 1),
                gregorian(1995, 5, 31),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1416,
            [
                gregorian(1995, 5, 31),
                gregorian(1995, 6, 30),
                gregorian(1995, 7, 29),
                gregorian(1995, 8, 28),
                gregorian(1995, 9, 26),
                gregorian(1995, 10, 26),
                gregorian(1995, 11, 24),
                gregorian(1995, 12, 23),
                gregorian(1996, 1, 22),
                gregorian(1996, 2, 20),
                gregorian(1996, 3, 21),
                gregorian(1996, 4, 19),
                gregorian(1996, 5, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1417,
            [
                gregorian(1996, 5, 19),
                gregorian(1996, 6, 18),
                gregorian(1996, 7, 17),
                gregorian(1996, 8, 16),
                gregorian(1996, 9, 15),
                gregorian(1996, 10, 14),
                gregorian(1996, 11, 12),
                gregorian(1996, 12, 12),
                gregorian(1997, 1, 10),
                gregorian(1997, 2, 9),
                gregorian(1997, 3, 10),
                gregorian(1997, 4, 9),
                gregorian(1997, 5, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1418,
            [
                gregorian(1997, 5, 8),
                gregorian(1997, 6, 7),
                gregorian(1997, 7, 6),
                gregorian(1997, 8, 5),
                gregorian(1997, 9, 4),
                gregorian(1997, 10, 3),
                gregorian(1997, 11, 2),
                gregorian(1997, 12, 1),
                gregorian(1997, 12, 31),
                gregorian(1998, 1, 29),
                gregorian(1998, 2, 28),
                gregorian(1998, 3, 29),
                gregorian(1998, 4, 28),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1419,
            [
                gregorian(1998, 4, 28),
                gregorian(1998, 5, 27),
                gregorian(1998, 6, 26),
                gregorian(1998, 7, 25),
                gregorian(1998, 8, 24),
                gregorian(1998, 9, 22),
                gregorian(1998, 10, 22),
                gregorian(1998, 11, 20),
                gregorian(1998, 12, 20),
                gregorian(1999, 1, 19),
                gregorian(1999, 2, 18),
                gregorian(1999, 3, 19),
                gregorian(1999, 4, 17),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1420,
            [
                gregorian(1999, 4, 17),
                gregorian(1999, 5, 16),
                gregorian(1999, 6, 15),
                gregorian(1999, 7, 14),
                gregorian(1999, 8, 12),
                gregorian(1999, 9, 11),
                gregorian(1999, 10, 10),
                gregorian(1999, 11, 9),
                gregorian(1999, 12, 9),
                gregorian(2000, 1, 8),
                gregorian(2000, 2, 7),
                gregorian(2000, 3, 7),
                gregorian(2000, 4, 6),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1421,
            [
                gregorian(2000, 4, 6),
                gregorian(2000, 5, 5),
                gregorian(2000, 6, 3),
                gregorian(2000, 7, 3),
                gregorian(2000, 8, 1),
                gregorian(2000, 8, 30),
                gregorian(2000, 9, 28),
                gregorian(2000, 10, 28),
                gregorian(2000, 11, 27),
                gregorian(2000, 12, 27),
                gregorian(2001, 1, 26),
                gregorian(2001, 2, 24),
                gregorian(2001, 3, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1422,
            [
                gregorian(2001, 3, 26),
                gregorian(2001, 4, 25),
                gregorian(2001, 5, 24),
                gregorian(2001, 6, 22),
                gregorian(2001, 7, 22),
                gregorian(2001, 8, 20),
                gregorian(2001, 9, 18),
                gregorian(2001, 10, 17),
                gregorian(2001, 11, 16),
                gregorian(2001, 12, 16),
                gregorian(2002, 1, 15),
                gregorian(2002, 2, 13),
                gregorian(2002, 3, 15),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1423,
            [
                gregorian(2002, 3, 15),
                gregorian(2002, 4, 14),
                gregorian(2002, 5, 13),
                gregorian(2002, 6, 12),
                gregorian(2002, 7, 11),
                gregorian(2002, 8, 10),
                gregorian(2002, 9, 8),
                gregorian(2002, 10, 7),
                gregorian(2002, 11, 6),
                gregorian(2002, 12, 5),
                gregorian(2003, 1, 4),
                gregorian(2003, 2, 2),
                gregorian(2003, 3, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1424,
            [
                gregorian(2003, 3, 4),
                gregorian(2003, 4, 3),
                gregorian(2003, 5, 2),
                gregorian(2003, 6, 1),
                gregorian(2003, 7, 1),
                gregorian(2003, 7, 30),
                gregorian(2003, 8, 29),
                gregorian(2003, 9, 27),
                gregorian(2003, 10, 26),
                gregorian(2003, 11, 25),
                gregorian(2003, 12, 24),
                gregorian(2004, 1, 23),
                gregorian(2004, 2, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1425,
            [
                gregorian(2004, 2, 21),
                gregorian(2004, 3, 22),
                gregorian(2004, 4, 20),
                gregorian(2004, 5, 20),
                gregorian(2004, 6, 19),
                gregorian(2004, 7, 18),
                gregorian(2004, 8, 17),
                gregorian(2004, 9, 15),
                gregorian(2004, 10, 15),
                gregorian(2004, 11, 14),
                gregorian(2004, 12, 13),
                gregorian(2005, 1, 12),
                gregorian(2005, 2, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1426,
            [
                gregorian(2005, 2, 10),
                gregorian(2005, 3, 11),
                gregorian(2005, 4, 10),
                gregorian(2005, 5, 9),
                gregorian(2005, 6, 8),
                gregorian(2005, 7, 7),
                gregorian(2005, 8, 6),
                gregorian(2005, 9, 5),
                gregorian(2005, 10, 4),
                gregorian(2005, 11, 3),
                gregorian(2005, 12, 3),
                gregorian(2006, 1, 1),
                gregorian(2006, 1, 31),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1427,
            [
                gregorian(2006, 1, 31),
                gregorian(2006, 3, 1),
                gregorian(2006, 3, 30),
                gregorian(2006, 4, 29),
                gregorian(2006, 5, 28),
                gregorian(2006, 6, 27),
                gregorian(2006, 7, 26),
                gregorian(2006, 8, 25),
                gregorian(2006, 9, 24),
                gregorian(2006, 10, 23),
                gregorian(2006, 11, 22),
                gregorian(2006, 12, 22),
                gregorian(2007, 1, 20),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1428,
            [
                gregorian(2007, 1, 20),
                gregorian(2007, 2, 19),
                gregorian(2007, 3, 20),
                gregorian(2007, 4, 18),
                gregorian(2007, 5, 18),
                gregorian(2007, 6, 16),
                gregorian(2007, 7, 15),
                gregorian(2007, 8, 14),
                gregorian(2007, 9, 13),
                gregorian(2007, 10, 13),
                gregorian(2007, 11, 11),
                gregorian(2007, 12, 11),
                gregorian(2008, 1, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1429,
            [
                gregorian(2008, 1, 10),
                gregorian(2008, 2, 8),
                gregorian(2008, 3, 9),
                gregorian(2008, 4, 7),
                gregorian(2008, 5, 6),
                gregorian(2008, 6, 5),
                gregorian(2008, 7, 4),
                gregorian(2008, 8, 2),
                gregorian(2008, 9, 1),
                gregorian(2008, 10, 1),
                gregorian(2008, 10, 30),
                gregorian(2008, 11, 29),
                gregorian(2008, 12, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1430,
            [
                gregorian(2008, 12, 29),
                gregorian(2009, 1, 27),
                gregorian(2009, 2, 26),
                gregorian(2009, 3, 28),
                gregorian(2009, 4, 26),
                gregorian(2009, 5, 25),
                gregorian(2009, 6, 24),
                gregorian(2009, 7, 23),
                gregorian(2009, 8, 22),
                gregorian(2009, 9, 20),
                gregorian(2009, 10, 20),
                gregorian(2009, 11, 18),
                gregorian(2009, 12, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1431,
            [
                gregorian(2009, 12, 18),
                gregorian(2010, 1, 16),
                gregorian(2010, 2, 15),
                gregorian(2010, 3, 17),
                gregorian(2010, 4, 15),
                gregorian(2010, 5, 15),
                gregorian(2010, 6, 13),
                gregorian(2010, 7, 13),
                gregorian(2010, 8, 11),
                gregorian(2010, 9, 10),
                gregorian(2010, 10, 9),
                gregorian(2010, 11, 7),
                gregorian(2010, 12, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1432,
            [
                gregorian(2010, 12, 7),
                gregorian(2011, 1, 5),
                gregorian(2011, 2, 4),
                gregorian(2011, 3, 6),
                gregorian(2011, 4, 5),
                gregorian(2011, 5, 4),
                gregorian(2011, 6, 3),
                gregorian(2011, 7, 2),
                gregorian(2011, 8, 1),
                gregorian(2011, 8, 30),
                gregorian(2011, 9, 29),
                gregorian(2011, 10, 28),
                gregorian(2011, 11, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1433,
            [
                gregorian(2011, 11, 26),
                gregorian(2011, 12, 26),
                gregorian(2012, 1, 24),
                gregorian(2012, 2, 23),
                gregorian(2012, 3, 24),
                gregorian(2012, 4, 22),
                gregorian(2012, 5, 22),
                gregorian(2012, 6, 21),
                gregorian(2012, 7, 20),
                gregorian(2012, 8, 19),
                gregorian(2012, 9, 17),
                gregorian(2012, 10, 17),
                gregorian(2012, 11, 15),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1434,
            [
                gregorian(2012, 11, 15),
                gregorian(2012, 12, 14),
                gregorian(2013, 1, 13),
                gregorian(2013, 2, 11),
                gregorian(2013, 3, 13),
                gregorian(2013, 4, 11),
                gregorian(2013, 5, 11),
                gregorian(2013, 6, 10),
                gregorian(2013, 7, 9),
                gregorian(2013, 8, 8),
                gregorian(2013, 9, 7),
                gregorian(2013, 10, 6),
                gregorian(2013, 11, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1435,
            [
                gregorian(2013, 11, 4),
                gregorian(2013, 12, 4),
                gregorian(2014, 1, 2),
                gregorian(2014, 2, 1),
                gregorian(2014, 3, 2),
                gregorian(2014, 4, 1),
                gregorian(2014, 4, 30),
                gregorian(2014, 5, 30),
                gregorian(2014, 6, 28),
                gregorian(2014, 7, 28),
                gregorian(2014, 8, 27),
                gregorian(2014, 9, 25),
                gregorian(2014, 10, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1436,
            [
                gregorian(2014, 10, 25),
                gregorian(2014, 11, 23),
                gregorian(2014, 12, 23),
                gregorian(2015, 1, 21),
                gregorian(2015, 2, 20),
                gregorian(2015, 3, 21),
                gregorian(2015, 4, 20),
                gregorian(2015, 5, 19),
                gregorian(2015, 6, 18),
                gregorian(2015, 7, 17),
                gregorian(2015, 8, 16),
                gregorian(2015, 9, 14),
                gregorian(2015, 10, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1437,
            [
                gregorian(2015, 10, 14),
                gregorian(2015, 11, 13),
                gregorian(2015, 12, 12),
                gregorian(2016, 1, 11),
                gregorian(2016, 2, 10),
                gregorian(2016, 3, 10),
                gregorian(2016, 4, 8),
                gregorian(2016, 5, 8),
                gregorian(2016, 6, 6),
                gregorian(2016, 7, 6),
                gregorian(2016, 8, 4),
                gregorian(2016, 9, 2),
                gregorian(2016, 10, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1438,
            [
                gregorian(2016, 10, 2),
                gregorian(2016, 11, 1),
                gregorian(2016, 11, 30),
                gregorian(2016, 12, 30),
                gregorian(2017, 1, 29),
                gregorian(2017, 2, 28),
                gregorian(2017, 3, 29),
                gregorian(2017, 4, 27),
                gregorian(2017, 5, 27),
                gregorian(2017, 6, 25),
                gregorian(2017, 7, 24),
                gregorian(2017, 8, 23),
                gregorian(2017, 9, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1439,
            [
                gregorian(2017, 9, 21),
                gregorian(2017, 10, 21),
                gregorian(2017, 11, 19),
                gregorian(2017, 12, 19),
                gregorian(2018, 1, 18),
                gregorian(2018, 2, 17),
                gregorian(2018, 3, 18),
                gregorian(2018, 4, 17),
                gregorian(2018, 5, 16),
                gregorian(2018, 6, 15),
                gregorian(2018, 7, 14),
                gregorian(2018, 8, 12),
                gregorian(2018, 9, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1440,
            [
                gregorian(2018, 9, 11),
                gregorian(2018, 10, 10),
                gregorian(2018, 11, 9),
                gregorian(2018, 12, 8),
                gregorian(2019, 1, 7),
                gregorian(2019, 2, 6),
                gregorian(2019, 3, 8),
                gregorian(2019, 4, 6),
                gregorian(2019, 5, 6),
                gregorian(2019, 6, 4),
                gregorian(2019, 7, 4),
                gregorian(2019, 8, 2),
                gregorian(2019, 8, 31),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1441,
            [
                gregorian(2019, 8, 31),
                gregorian(2019, 9, 30),
                gregorian(2019, 10, 29),
                gregorian(2019, 11, 28),
                gregorian(2019, 12, 27),
                gregorian(2020, 1, 26),
                gregorian(2020, 2, 25),
                gregorian(2020, 3, 25),
                gregorian(2020, 4, 24),
                gregorian(2020, 5, 24),
                gregorian(2020, 6, 22),
                gregorian(2020, 7, 22),
                gregorian(2020, 8, 20),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1442,
            [
                gregorian(2020, 8, 20),
                gregorian(2020, 9, 18),
                gregorian(2020, 10, 18),
                gregorian(2020, 11, 16),
                gregorian(2020, 12, 16),
                gregorian(2021, 1, 14),
                gregorian(2021, 2, 13),
                gregorian(2021, 3, 14),
                gregorian(2021, 4, 13),
                gregorian(2021, 5, 13),
                gregorian(2021, 6, 11),
                gregorian(2021, 7, 11),
                gregorian(2021, 8, 9),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1443,
            [
                gregorian(2021, 8, 9),
                gregorian(2021, 9, 8),
                gregorian(2021, 10, 7),
                gregorian(2021, 11, 6),
                gregorian(2021, 12, 5),
                gregorian(2022, 1, 4),
                gregorian(2022, 2, 2),
                gregorian(2022, 3, 4),
                gregorian(2022, 4, 2),
                gregorian(2022, 5, 2),
                gregorian(2022, 5, 31),
                gregorian(2022, 6, 30),
                gregorian(2022, 7, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1444,
            [
                gregorian(2022, 7, 30),
                gregorian(2022, 8, 28),
                gregorian(2022, 9, 27),
                gregorian(2022, 10, 26),
                gregorian(2022, 11, 25),
                gregorian(2022, 12, 25),
                gregorian(2023, 1, 23),
                gregorian(2023, 2, 21),
                gregorian(2023, 3, 23),
                gregorian(2023, 4, 21),
                gregorian(2023, 5, 21),
                gregorian(2023, 6, 19),
                gregorian(2023, 7, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1445,
            [
                gregorian(2023, 7, 19),
                gregorian(2023, 8, 17),
                gregorian(2023, 9, 16),
                gregorian(2023, 10, 16),
                gregorian(2023, 11, 15),
                gregorian(2023, 12, 14),
                gregorian(2024, 1, 13),
                gregorian(2024, 2, 11),
                gregorian(2024, 3, 11),
                gregorian(2024, 4, 10),
                gregorian(2024, 5, 9),
                gregorian(2024, 6, 7),
                gregorian(2024, 7, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1446,
            [
                gregorian(2024, 7, 7),
                gregorian(2024, 8, 5),
                gregorian(2024, 9, 4),
                gregorian(2024, 10, 4),
                gregorian(2024, 11, 3),
                gregorian(2024, 12, 2),
                gregorian(2025, 1, 1),
                gregorian(2025, 1, 31),
                gregorian(2025, 3, 1),
                gregorian(2025, 3, 30),
                gregorian(2025, 4, 29),
                gregorian(2025, 5, 28),
                gregorian(2025, 6, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1447,
            [
                gregorian(2025, 6, 26),
                gregorian(2025, 7, 26),
                gregorian(2025, 8, 24),
                gregorian(2025, 9, 23),
                gregorian(2025, 10, 23),
                gregorian(2025, 11, 22),
                gregorian(2025, 12, 21),
                gregorian(2026, 1, 20),
                gregorian(2026, 2, 18),
                gregorian(2026, 3, 20),
                gregorian(2026, 4, 18),
                gregorian(2026, 5, 18),
                gregorian(2026, 6, 16),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1448,
            [
                gregorian(2026, 6, 16),
                gregorian(2026, 7, 15),
                gregorian(2026, 8, 14),
                gregorian(2026, 9, 12),
                gregorian(2026, 10, 12),
                gregorian(2026, 11, 11),
                gregorian(2026, 12, 10),
                gregorian(2027, 1, 9),
                gregorian(2027, 2, 8),
                gregorian(2027, 3, 9),
                gregorian(2027, 4, 8),
                gregorian(2027, 5, 7),
                gregorian(2027, 6, 6),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1449,
            [
                gregorian(2027, 6, 6),
                gregorian(2027, 7, 5),
                gregorian(2027, 8, 3),
                gregorian(2027, 9, 2),
                gregorian(2027, 10, 1),
                gregorian(2027, 10, 31),
                gregorian(2027, 11, 29),
                gregorian(2027, 12, 29),
                gregorian(2028, 1, 28),
                gregorian(2028, 2, 26),
                gregorian(2028, 3, 27),
                gregorian(2028, 4, 26),
                gregorian(2028, 5, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1450,
            [
                gregorian(2028, 5, 25),
                gregorian(2028, 6, 24),
                gregorian(2028, 7, 23),
                gregorian(2028, 8, 22),
                gregorian(2028, 9, 20),
                gregorian(2028, 10, 19),
                gregorian(2028, 11, 18),
                gregorian(2028, 12, 17),
                gregorian(2029, 1, 16),
                gregorian(2029, 2, 14),
                gregorian(2029, 3, 16),
                gregorian(2029, 4, 15),
                gregorian(2029, 5, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1451,
            [
                gregorian(2029, 5, 14),
                gregorian(2029, 6, 13),
                gregorian(2029, 7, 13),
                gregorian(2029, 8, 12),
                gregorian(2029, 9, 10),
                gregorian(2029, 10, 9),
                gregorian(2029, 11, 8),
                gregorian(2029, 12, 7),
                gregorian(2030, 1, 5),
                gregorian(2030, 2, 4),
                gregorian(2030, 3, 6),
                gregorian(2030, 4, 4),
                gregorian(2030, 5, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1452,
            [
                gregorian(2030, 5, 4),
                gregorian(2030, 6, 3),
                gregorian(2030, 7, 2),
                gregorian(2030, 8, 1),
                gregorian(2030, 8, 31),
                gregorian(2030, 9, 29),
                gregorian(2030, 10, 28),
                gregorian(2030, 11, 27),
                gregorian(2030, 12, 26),
                gregorian(2031, 1, 24),
                gregorian(2031, 2, 23),
                gregorian(2031, 3, 24),
                gregorian(2031, 4, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1453,
            [
                gregorian(2031, 4, 23),
                gregorian(2031, 5, 23),
                gregorian(2031, 6, 21),
                gregorian(2031, 7, 21),
                gregorian(2031, 8, 20),
                gregorian(2031, 9, 18),
                gregorian(2031, 10, 18),
                gregorian(2031, 11, 16),
                gregorian(2031, 12, 16),
                gregorian(2032, 1, 14),
                gregorian(2032, 2, 12),
                gregorian(2032, 3, 13),
                gregorian(2032, 4, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1454,
            [
                gregorian(2032, 4, 11),
                gregorian(2032, 5, 11),
                gregorian(2032, 6, 9),
                gregorian(2032, 7, 9),
                gregorian(2032, 8, 8),
                gregorian(2032, 9, 6),
                gregorian(2032, 10, 6),
                gregorian(2032, 11, 5),
                gregorian(2032, 12, 4),
                gregorian(2033, 1, 3),
                gregorian(2033, 2, 1),
                gregorian(2033, 3, 3),
                gregorian(2033, 4, 1),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1455,
            [
                gregorian(2033, 4, 1),
                gregorian(2033, 4, 30),
                gregorian(2033, 5, 30),
                gregorian(2033, 6, 28),
                gregorian(2033, 7, 28),
                gregorian(2033, 8, 27),
                gregorian(2033, 9, 25),
                gregorian(2033, 10, 25),
                gregorian(2033, 11, 23),
                gregorian(2033, 12, 23),
                gregorian(2034, 1, 22),
                gregorian(2034, 2, 20),
                gregorian(2034, 3, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1456,
            [
                gregorian(2034, 3, 22),
                gregorian(2034, 4, 20),
                gregorian(2034, 5, 19),
                gregorian(2034, 6, 18),
                gregorian(2034, 7, 17),
                gregorian(2034, 8, 16),
                gregorian(2034, 9, 14),
                gregorian(2034, 10, 14),
                gregorian(2034, 11, 12),
                gregorian(2034, 12, 12),
                gregorian(2035, 1, 11),
                gregorian(2035, 2, 10),
                gregorian(2035, 3, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1457,
            [
                gregorian(2035, 3, 11),
                gregorian(2035, 4, 10),
                gregorian(2035, 5, 9),
                gregorian(2035, 6, 7),
                gregorian(2035, 7, 7),
                gregorian(2035, 8, 5),
                gregorian(2035, 9, 3),
                gregorian(2035, 10, 3),
                gregorian(2035, 11, 1),
                gregorian(2035, 12, 1),
                gregorian(2035, 12, 31),
                gregorian(2036, 1, 30),
                gregorian(2036, 2, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1458,
            [
                gregorian(2036, 2, 29),
                gregorian(2036, 3, 29),
                gregorian(2036, 4, 28),
                gregorian(2036, 5, 27),
                gregorian(2036, 6, 25),
                gregorian(2036, 7, 25),
                gregorian(2036, 8, 23),
                gregorian(2036, 9, 21),
                gregorian(2036, 10, 21),
                gregorian(2036, 11, 19),
                gregorian(2036, 12, 19),
                gregorian(2037, 1, 18),
                gregorian(2037, 2, 17),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1459,
            [
                gregorian(2037, 2, 17),
                gregorian(2037, 3, 18),
                gregorian(2037, 4, 17),
                gregorian(2037, 5, 17),
                gregorian(2037, 6, 15),
                gregorian(2037, 7, 14),
                gregorian(2037, 8, 13),
                gregorian(2037, 9, 11),
                gregorian(2037, 10, 10),
                gregorian(2037, 11, 9),
                gregorian(2037, 12, 8),
                gregorian(2038, 1, 7),
                gregorian(2038, 2, 6),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1460,
            [
                gregorian(2038, 2, 6),
                gregorian(2038, 3, 7),
                gregorian(2038, 4, 6),
                gregorian(2038, 5, 6),
                gregorian(2038, 6, 4),
                gregorian(2038, 7, 4),
                gregorian(2038, 8, 2),
                gregorian(2038, 9, 1),
                gregorian(2038, 9, 30),
                gregorian(2038, 10, 29),
                gregorian(2038, 11, 28),
                gregorian(2038, 12, 27),
                gregorian(2039, 1, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1461,
            [
                gregorian(2039, 1, 26),
                gregorian(2039, 2, 24),
                gregorian(2039, 3, 26),
                gregorian(2039, 4, 25),
                gregorian(2039, 5, 24),
                gregorian(2039, 6, 23),
                gregorian(2039, 7, 22),
                gregorian(2039, 8, 21),
                gregorian(2039, 9, 19),
                gregorian(2039, 10, 19),
                gregorian(2039, 11, 18),
                gregorian(2039, 12, 17),
                gregorian(2040, 1, 15),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1462,
            [
                gregorian(2040, 1, 15),
                gregorian(2040, 2, 14),
                gregorian(2040, 3, 14),
                gregorian(2040, 4, 13),
                gregorian(2040, 5, 12),
                gregorian(2040, 6, 11),
                gregorian(2040, 7, 11),
                gregorian(2040, 8, 9),
                gregorian(2040, 9, 8),
                gregorian(2040, 10, 7),
                gregorian(2040, 11, 6),
                gregorian(2040, 12, 6),
                gregorian(2041, 1, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1463,
            [
                gregorian(2041, 1, 4),
                gregorian(2041, 2, 2),
                gregorian(2041, 3, 4),
                gregorian(2041, 4, 2),
                gregorian(2041, 5, 2),
                gregorian(2041, 5, 31),
                gregorian(2041, 6, 30),
                gregorian(2041, 7, 29),
                gregorian(2041, 8, 28),
                gregorian(2041, 9, 27),
                gregorian(2041, 10, 27),
                gregorian(2041, 11, 25),
                gregorian(2041, 12, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1464,
            [
                gregorian(2041, 12, 25),
                gregorian(2042, 1, 23),
                gregorian(2042, 2, 22),
                gregorian(2042, 3, 23),
                gregorian(2042, 4, 21),
                gregorian(2042, 5, 21),
                gregorian(2042, 6, 19),
                gregorian(2042, 7, 18),
                gregorian(2042, 8, 17),
                gregorian(2042, 9, 16),
                gregorian(2042, 10, 16),
                gregorian(2042, 11, 14),
                gregorian(2042, 12, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1465,
            [
                gregorian(2042, 12, 14),
                gregorian(2043, 1, 13),
                gregorian(2043, 2, 11),
                gregorian(2043, 3, 13),
                gregorian(2043, 4, 11),
                gregorian(2043, 5, 10),
                gregorian(2043, 6, 9),
                gregorian(2043, 7, 8),
                gregorian(2043, 8, 6),
                gregorian(2043, 9, 5),
                gregorian(2043, 10, 5),
                gregorian(2043, 11, 3),
                gregorian(2043, 12, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1466,
            [
                gregorian(2043, 12, 3),
                gregorian(2044, 1, 2),
                gregorian(2044, 2, 1),
                gregorian(2044, 3, 1),
                gregorian(2044, 3, 31),
                gregorian(2044, 4, 29),
                gregorian(2044, 5, 28),
                gregorian(2044, 6, 26),
                gregorian(2044, 7, 26),
                gregorian(2044, 8, 24),
                gregorian(2044, 9, 23),
                gregorian(2044, 10, 23),
                gregorian(2044, 11, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1467,
            [
                gregorian(2044, 11, 21),
                gregorian(2044, 12, 21),
                gregorian(2045, 1, 20),
                gregorian(2045, 2, 18),
                gregorian(2045, 3, 20),
                gregorian(2045, 4, 19),
                gregorian(2045, 5, 18),
                gregorian(2045, 6, 16),
                gregorian(2045, 7, 16),
                gregorian(2045, 8, 14),
                gregorian(2045, 9, 13),
                gregorian(2045, 10, 12),
                gregorian(2045, 11, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1468,
            [
                gregorian(2045, 11, 11),
                gregorian(2045, 12, 10),
                gregorian(2046, 1, 9),
                gregorian(2046, 2, 7),
                gregorian(2046, 3, 9),
                gregorian(2046, 4, 8),
                gregorian(2046, 5, 7),
                gregorian(2046, 6, 6),
                gregorian(2046, 7, 5),
                gregorian(2046, 8, 4),
                gregorian(2046, 9, 2),
                gregorian(2046, 10, 2),
                gregorian(2046, 10, 31),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1469,
            [
                gregorian(2046, 10, 31),
                gregorian(2046, 11, 29),
                gregorian(2046, 12, 29),
                gregorian(2047, 1, 27),
                gregorian(2047, 2, 26),
                gregorian(2047, 3, 28),
                gregorian(2047, 4, 26),
                gregorian(2047, 5, 26),
                gregorian(2047, 6, 25),
                gregorian(2047, 7, 24),
                gregorian(2047, 8, 23),
                gregorian(2047, 9, 21),
                gregorian(2047, 10, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1470,
            [
                gregorian(2047, 10, 21),
                gregorian(2047, 11, 19),
                gregorian(2047, 12, 18),
                gregorian(2048, 1, 17),
                gregorian(2048, 2, 15),
                gregorian(2048, 3, 16),
                gregorian(2048, 4, 15),
                gregorian(2048, 5, 14),
                gregorian(2048, 6, 13),
                gregorian(2048, 7, 13),
                gregorian(2048, 8, 11),
                gregorian(2048, 9, 10),
                gregorian(2048, 10, 9),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1471,
            [
                gregorian(2048, 10, 9),
                gregorian(2048, 11, 8),
                gregorian(2048, 12, 7),
                gregorian(2049, 1, 5),
                gregorian(2049, 2, 4),
                gregorian(2049, 3, 5),
                gregorian(2049, 4, 4),
                gregorian(2049, 5, 3),
                gregorian(2049, 6, 2),
                gregorian(2049, 7, 2),
                gregorian(2049, 7, 31),
                gregorian(2049, 8, 30),
                gregorian(2049, 9, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1472,
            [
                gregorian(2049, 9, 29),
                gregorian(2049, 10, 28),
                gregorian(2049, 11, 27),
                gregorian(2049, 12, 26),
                gregorian(2050, 1, 24),
                gregorian(2050, 2, 23),
                gregorian(2050, 3, 24),
                gregorian(2050, 4, 23),
                gregorian(2050, 5, 22),
                gregorian(2050, 6, 21),
                gregorian(2050, 7, 21),
                gregorian(2050, 8, 19),
                gregorian(2050, 9, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1473,
            [
                gregorian(2050, 9, 18),
                gregorian(2050, 10, 17),
                gregorian(2050, 11, 16),
                gregorian(2050, 12, 15),
                gregorian(2051, 1, 14),
                gregorian(2051, 2, 13),
                gregorian(2051, 3, 14),
                gregorian(2051, 4, 12),
                gregorian(2051, 5, 12),
                gregorian(2051, 6, 10),
                gregorian(2051, 7, 10),
                gregorian(2051, 8, 8),
                gregorian(2051, 9, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1474,
            [
                gregorian(2051, 9, 7),
                gregorian(2051, 10, 6),
                gregorian(2051, 11, 5),
                gregorian(2051, 12, 5),
                gregorian(2052, 1, 3),
                gregorian(2052, 2, 2),
                gregorian(2052, 3, 3),
                gregorian(2052, 4, 1),
                gregorian(2052, 4, 30),
                gregorian(2052, 5, 30),
                gregorian(2052, 6, 28),
                gregorian(2052, 7, 28),
                gregorian(2052, 8, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1475,
            [
                gregorian(2052, 8, 26),
                gregorian(2052, 9, 24),
                gregorian(2052, 10, 24),
                gregorian(2052, 11, 23),
                gregorian(2052, 12, 22),
                gregorian(2053, 1, 21),
                gregorian(2053, 2, 20),
                gregorian(2053, 3, 22),
                gregorian(2053, 4, 20),
                gregorian(2053, 5, 19),
                gregorian(2053, 6, 18),
                gregorian(2053, 7, 17),
                gregorian(2053, 8, 15),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1476,
            [
                gregorian(2053, 8, 15),
                gregorian(2053, 9, 14),
                gregorian(2053, 10, 13),
                gregorian(2053, 11, 12),
                gregorian(2053, 12, 11),
                gregorian(2054, 1, 10),
                gregorian(2054, 2, 9),
                gregorian(2054, 3, 11),
                gregorian(2054, 4, 9),
                gregorian(2054, 5, 9),
                gregorian(2054, 6, 7),
                gregorian(2054, 7, 7),
                gregorian(2054, 8, 5),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1477,
            [
                gregorian(2054, 8, 5),
                gregorian(2054, 9, 3),
                gregorian(2054, 10, 3),
                gregorian(2054, 11, 1),
                gregorian(2054, 11, 30),
                gregorian(2054, 12, 30),
                gregorian(2055, 1, 29),
                gregorian(2055, 2, 28),
                gregorian(2055, 3, 30),
                gregorian(2055, 4, 28),
                gregorian(2055, 5, 28),
                gregorian(2055, 6, 26),
                gregorian(2055, 7, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1478,
            [
                gregorian(2055, 7, 26),
                gregorian(2055, 8, 24),
                gregorian(2055, 9, 22),
                gregorian(2055, 10, 22),
                gregorian(2055, 11, 20),
                gregorian(2055, 12, 20),
                gregorian(2056, 1, 18),
                gregorian(2056, 2, 17),
                gregorian(2056, 3, 18),
                gregorian(2056, 4, 16),
                gregorian(2056, 5, 16),
                gregorian(2056, 6, 15),
                gregorian(2056, 7, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1479,
            [
                gregorian(2056, 7, 14),
                gregorian(2056, 8, 13),
                gregorian(2056, 9, 11),
                gregorian(2056, 10, 10),
                gregorian(2056, 11, 9),
                gregorian(2056, 12, 8),
                gregorian(2057, 1, 7),
                gregorian(2057, 2, 5),
                gregorian(2057, 3, 7),
                gregorian(2057, 4, 5),
                gregorian(2057, 5, 5),
                gregorian(2057, 6, 4),
                gregorian(2057, 7, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1480,
            [
                gregorian(2057, 7, 3),
                gregorian(2057, 8, 2),
                gregorian(2057, 8, 31),
                gregorian(2057, 9, 30),
                gregorian(2057, 10, 29),
                gregorian(2057, 11, 28),
                gregorian(2057, 12, 27),
                gregorian(2058, 1, 26),
                gregorian(2058, 2, 24),
                gregorian(2058, 3, 26),
                gregorian(2058, 4, 24),
                gregorian(2058, 5, 24),
                gregorian(2058, 6, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1481,
            [
                gregorian(2058, 6, 22),
                gregorian(2058, 7, 22),
                gregorian(2058, 8, 20),
                gregorian(2058, 9, 19),
                gregorian(2058, 10, 19),
                gregorian(2058, 11, 17),
                gregorian(2058, 12, 17),
                gregorian(2059, 1, 15),
                gregorian(2059, 2, 14),
                gregorian(2059, 3, 15),
                gregorian(2059, 4, 14),
                gregorian(2059, 5, 13),
                gregorian(2059, 6, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1482,
            [
                gregorian(2059, 6, 11),
                gregorian(2059, 7, 11),
                gregorian(2059, 8, 9),
                gregorian(2059, 9, 8),
                gregorian(2059, 10, 8),
                gregorian(2059, 11, 7),
                gregorian(2059, 12, 7),
                gregorian(2060, 1, 5),
                gregorian(2060, 2, 4),
                gregorian(2060, 3, 4),
                gregorian(2060, 4, 2),
                gregorian(2060, 5, 2),
                gregorian(2060, 5, 31),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1483,
            [
                gregorian(2060, 5, 31),
                gregorian(2060, 6, 29),
                gregorian(2060, 7, 29),
                gregorian(2060, 8, 27),
                gregorian(2060, 9, 26),
                gregorian(2060, 10, 26),
                gregorian(2060, 11, 25),
                gregorian(2060, 12, 24),
                gregorian(2061, 1, 23),
                gregorian(2061, 2, 22),
                gregorian(2061, 3, 23),
                gregorian(2061, 4, 21),
                gregorian(2061, 5, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1484,
            [
                gregorian(2061, 5, 21),
                gregorian(2061, 6, 19),
                gregorian(2061, 7, 18),
                gregorian(2061, 8, 17),
                gregorian(2061, 9, 15),
                gregorian(2061, 10, 15),
                gregorian(2061, 11, 14),
                gregorian(2061, 12, 14),
                gregorian(2062, 1, 12),
                gregorian(2062, 2, 11),
                gregorian(2062, 3, 12),
                gregorian(2062, 4, 11),
                gregorian(2062, 5, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1485,
            [
                gregorian(2062, 5, 10),
                gregorian(2062, 6, 9),
                gregorian(2062, 7, 8),
                gregorian(2062, 8, 6),
                gregorian(2062, 9, 5),
                gregorian(2062, 10, 4),
                gregorian(2062, 11, 3),
                gregorian(2062, 12, 3),
                gregorian(2063, 1, 1),
                gregorian(2063, 1, 31),
                gregorian(2063, 3, 2),
                gregorian(2063, 3, 31),
                gregorian(2063, 4, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1486,
            [
                gregorian(2063, 4, 30),
                gregorian(2063, 5, 29),
                gregorian(2063, 6, 28),
                gregorian(2063, 7, 27),
                gregorian(2063, 8, 25),
                gregorian(2063, 9, 24),
                gregorian(2063, 10, 23),
                gregorian(2063, 11, 22),
                gregorian(2063, 12, 21),
                gregorian(2064, 1, 20),
                gregorian(2064, 2, 19),
                gregorian(2064, 3, 19),
                gregorian(2064, 4, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1487,
            [
                gregorian(2064, 4, 18),
                gregorian(2064, 5, 18),
                gregorian(2064, 6, 16),
                gregorian(2064, 7, 16),
                gregorian(2064, 8, 14),
                gregorian(2064, 9, 13),
                gregorian(2064, 10, 12),
                gregorian(2064, 11, 10),
                gregorian(2064, 12, 10),
                gregorian(2065, 1, 8),
                gregorian(2065, 2, 7),
                gregorian(2065, 3, 8),
                gregorian(2065, 4, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1488,
            [
                gregorian(2065, 4, 7),
                gregorian(2065, 5, 7),
                gregorian(2065, 6, 5),
                gregorian(2065, 7, 5),
                gregorian(2065, 8, 4),
                gregorian(2065, 9, 2),
                gregorian(2065, 10, 2),
                gregorian(2065, 10, 31),
                gregorian(2065, 11, 29),
                gregorian(2065, 12, 29),
                gregorian(2066, 1, 27),
                gregorian(2066, 2, 26),
                gregorian(2066, 3, 27),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1489,
            [
                gregorian(2066, 3, 27),
                gregorian(2066, 4, 26),
                gregorian(2066, 5, 25),
                gregorian(2066, 6, 24),
                gregorian(2066, 7, 24),
                gregorian(2066, 8, 23),
                gregorian(2066, 9, 21),
                gregorian(2066, 10, 21),
                gregorian(2066, 11, 19),
                gregorian(2066, 12, 18),
                gregorian(2067, 1, 17),
                gregorian(2067, 2, 15),
                gregorian(2067, 3, 17),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1490,
            [
                gregorian(2067, 3, 17),
                gregorian(2067, 4, 15),
                gregorian(2067, 5, 15),
                gregorian(2067, 6, 13),
                gregorian(2067, 7, 13),
                gregorian(2067, 8, 12),
                gregorian(2067, 9, 10),
                gregorian(2067, 10, 10),
                gregorian(2067, 11, 9),
                gregorian(2067, 12, 8),
                gregorian(2068, 1, 6),
                gregorian(2068, 2, 5),
                gregorian(2068, 3, 5),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1491,
            [
                gregorian(2068, 3, 5),
                gregorian(2068, 4, 4),
                gregorian(2068, 5, 3),
                gregorian(2068, 6, 1),
                gregorian(2068, 7, 1),
                gregorian(2068, 7, 31),
                gregorian(2068, 8, 29),
                gregorian(2068, 9, 28),
                gregorian(2068, 10, 28),
                gregorian(2068, 11, 26),
                gregorian(2068, 12, 26),
                gregorian(2069, 1, 24),
                gregorian(2069, 2, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1492,
            [
                gregorian(2069, 2, 23),
                gregorian(2069, 3, 24),
                gregorian(2069, 4, 23),
                gregorian(2069, 5, 22),
                gregorian(2069, 6, 20),
                gregorian(2069, 7, 20),
                gregorian(2069, 8, 19),
                gregorian(2069, 9, 17),
                gregorian(2069, 10, 17),
                gregorian(2069, 11, 15),
                gregorian(2069, 12, 15),
                gregorian(2070, 1, 14),
                gregorian(2070, 2, 12),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1493,
            [
                gregorian(2070, 2, 12),
                gregorian(2070, 3, 14),
                gregorian(2070, 4, 12),
                gregorian(2070, 5, 12),
                gregorian(2070, 6, 10),
                gregorian(2070, 7, 10),
                gregorian(2070, 8, 8),
                gregorian(2070, 9, 6),
                gregorian(2070, 10, 6),
                gregorian(2070, 11, 4),
                gregorian(2070, 12, 4),
                gregorian(2071, 1, 3),
                gregorian(2071, 2, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1494,
            [
                gregorian(2071, 2, 2),
                gregorian(2071, 3, 3),
                gregorian(2071, 4, 2),
                gregorian(2071, 5, 1),
                gregorian(2071, 5, 31),
                gregorian(2071, 6, 29),
                gregorian(2071, 7, 29),
                gregorian(2071, 8, 27),
                gregorian(2071, 9, 25),
                gregorian(2071, 10, 24),
                gregorian(2071, 11, 23),
                gregorian(2071, 12, 23),
                gregorian(2072, 1, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1495,
            [
                gregorian(2072, 1, 22),
                gregorian(2072, 2, 20),
                gregorian(2072, 3, 21),
                gregorian(2072, 4, 20),
                gregorian(2072, 5, 19),
                gregorian(2072, 6, 18),
                gregorian(2072, 7, 17),
                gregorian(2072, 8, 15),
                gregorian(2072, 9, 14),
                gregorian(2072, 10, 13),
                gregorian(2072, 11, 11),
                gregorian(2072, 12, 11),
                gregorian(2073, 1, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1496,
            [
                gregorian(2073, 1, 10),
                gregorian(2073, 2, 8),
                gregorian(2073, 3, 10),
                gregorian(2073, 4, 9),
                gregorian(2073, 5, 9),
                gregorian(2073, 6, 7),
                gregorian(2073, 7, 7),
                gregorian(2073, 8, 5),
                gregorian(2073, 9, 3),
                gregorian(2073, 10, 3),
                gregorian(2073, 11, 1),
                gregorian(2073, 11, 30),
                gregorian(2073, 12, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1497,
            [
                gregorian(2073, 12, 30),
                gregorian(2074, 1, 29),
                gregorian(2074, 2, 27),
                gregorian(2074, 3, 29),
                gregorian(2074, 4, 28),
                gregorian(2074, 5, 27),
                gregorian(2074, 6, 26),
                gregorian(2074, 7, 25),
                gregorian(2074, 8, 24),
                gregorian(2074, 9, 22),
                gregorian(2074, 10, 22),
                gregorian(2074, 11, 20),
                gregorian(2074, 12, 20),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1498,
            [
                gregorian(2074, 12, 20),
                gregorian(2075, 1, 18),
                gregorian(2075, 2, 17),
                gregorian(2075, 3, 18),
                gregorian(2075, 4, 17),
                gregorian(2075, 5, 16),
                gregorian(2075, 6, 15),
                gregorian(2075, 7, 15),
                gregorian(2075, 8, 13),
                gregorian(2075, 9, 12),
                gregorian(2075, 10, 11),
                gregorian(2075, 11, 10),
                gregorian(2075, 12, 9),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1499,
            [
                gregorian(2075, 12, 9),
                gregorian(2076, 1, 8),
                gregorian(2076, 2, 6),
                gregorian(2076, 3, 7),
                gregorian(2076, 4, 5),
                gregorian(2076, 5, 4),
                gregorian(2076, 6, 3),
                gregorian(2076, 7, 3),
                gregorian(2076, 8, 1),
                gregorian(2076, 8, 31),
                gregorian(2076, 9, 29),
                gregorian(2076, 10, 29),
                gregorian(2076, 11, 28),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1500,
            [
                gregorian(2076, 11, 28),
                gregorian(2076, 12, 27),
                gregorian(2077, 1, 26),
                gregorian(2077, 2, 24),
                gregorian(2077, 3, 26),
                gregorian(2077, 4, 24),
                gregorian(2077, 5, 23),
                gregorian(2077, 6, 22),
                gregorian(2077, 7, 21),
                gregorian(2077, 8, 20),
                gregorian(2077, 9, 18),
                gregorian(2077, 10, 18),
                gregorian(2077, 11, 17),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1501,
            [
                gregorian(2077, 11, 17),
                gregorian(2077, 12, 17),
                gregorian(2078, 1, 15),
                gregorian(2078, 2, 14),
                gregorian(2078, 3, 15),
                gregorian(2078, 4, 14),
                gregorian(2078, 5, 13),
                gregorian(2078, 6, 11),
                gregorian(2078, 7, 10),
                gregorian(2078, 8, 9),
                gregorian(2078, 9, 7),
                gregorian(2078, 10, 7),
                gregorian(2078, 11, 6),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1502,
            [
                gregorian(2078, 11, 6),
                gregorian(2078, 12, 6),
                gregorian(2079, 1, 5),
                gregorian(2079, 2, 3),
                gregorian(2079, 3, 5),
                gregorian(2079, 4, 3),
                gregorian(2079, 5, 3),
                gregorian(2079, 6, 1),
                gregorian(2079, 6, 30),
                gregorian(2079, 7, 29),
                gregorian(2079, 8, 28),
                gregorian(2079, 9, 27),
                gregorian(2079, 10, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1503,
            [
                gregorian(2079, 10, 26),
                gregorian(2079, 11, 25),
                gregorian(2079, 12, 25),
                gregorian(2080, 1, 23),
                gregorian(2080, 2, 22),
                gregorian(2080, 3, 23),
                gregorian(2080, 4, 21),
                gregorian(2080, 5, 21),
                gregorian(2080, 6, 19),
                gregorian(2080, 7, 18),
                gregorian(2080, 8, 16),
                gregorian(2080, 9, 15),
                gregorian(2080, 10, 15),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1504,
            [
                gregorian(2080, 10, 15),
                gregorian(2080, 11, 13),
                gregorian(2080, 12, 13),
                gregorian(2081, 1, 11),
                gregorian(2081, 2, 10),
                gregorian(2081, 3, 12),
                gregorian(2081, 4, 11),
                gregorian(2081, 5, 10),
                gregorian(2081, 6, 8),
                gregorian(2081, 7, 8),
                gregorian(2081, 8, 6),
                gregorian(2081, 9, 5),
                gregorian(2081, 10, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1505,
            [
                gregorian(2081, 10, 4),
                gregorian(2081, 11, 3),
                gregorian(2081, 12, 2),
                gregorian(2082, 1, 1),
                gregorian(2082, 1, 30),
                gregorian(2082, 3, 1),
                gregorian(2082, 3, 31),
                gregorian(2082, 4, 29),
                gregorian(2082, 5, 29),
                gregorian(2082, 6, 27),
                gregorian(2082, 7, 27),
                gregorian(2082, 8, 26),
                gregorian(2082, 9, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1506,
            [
                gregorian(2082, 9, 24),
                gregorian(2082, 10, 23),
                gregorian(2082, 11, 22),
                gregorian(2082, 12, 21),
                gregorian(2083, 1, 19),
                gregorian(2083, 2, 18),
                gregorian(2083, 3, 20),
                gregorian(2083, 4, 18),
                gregorian(2083, 5, 18),
                gregorian(2083, 6, 17),
                gregorian(2083, 7, 16),
                gregorian(2083, 8, 15),
                gregorian(2083, 9, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1507,
            [
                gregorian(2083, 9, 14),
                gregorian(2083, 10, 13),
                gregorian(2083, 11, 11),
                gregorian(2083, 12, 11),
                gregorian(2084, 1, 9),
                gregorian(2084, 2, 7),
                gregorian(2084, 3, 8),
                gregorian(2084, 4, 7),
                gregorian(2084, 5, 6),
                gregorian(2084, 6, 5),
                gregorian(2084, 7, 4),
                gregorian(2084, 8, 3),
                gregorian(2084, 9, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1508,
            [
                gregorian(2084, 9, 2),
                gregorian(2084, 10, 2),
                gregorian(2084, 10, 31),
                gregorian(2084, 11, 29),
                gregorian(2084, 12, 29),
                gregorian(2085, 1, 27),
                gregorian(2085, 2, 26),
                gregorian(2085, 3, 27),
                gregorian(2085, 4, 25),
                gregorian(2085, 5, 25),
                gregorian(2085, 6, 23),
                gregorian(2085, 7, 23),
                gregorian(2085, 8, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1509,
            [
                gregorian(2085, 8, 22),
                gregorian(2085, 9, 21),
                gregorian(2085, 10, 20),
                gregorian(2085, 11, 19),
                gregorian(2085, 12, 18),
                gregorian(2086, 1, 17),
                gregorian(2086, 2, 15),
                gregorian(2086, 3, 17),
                gregorian(2086, 4, 15),
                gregorian(2086, 5, 14),
                gregorian(2086, 6, 13),
                gregorian(2086, 7, 12),
                gregorian(2086, 8, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1510,
            [
                gregorian(2086, 8, 11),
                gregorian(2086, 9, 10),
                gregorian(2086, 10, 9),
                gregorian(2086, 11, 8),
                gregorian(2086, 12, 8),
                gregorian(2087, 1, 6),
                gregorian(2087, 2, 5),
                gregorian(2087, 3, 6),
                gregorian(2087, 4, 5),
                gregorian(2087, 5, 4),
                gregorian(2087, 6, 2),
                gregorian(2087, 7, 2),
                gregorian(2087, 7, 31),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1511,
            [
                gregorian(2087, 7, 31),
                gregorian(2087, 8, 30),
                gregorian(2087, 9, 28),
                gregorian(2087, 10, 28),
                gregorian(2087, 11, 27),
                gregorian(2087, 12, 26),
                gregorian(2088, 1, 25),
                gregorian(2088, 2, 24),
                gregorian(2088, 3, 24),
                gregorian(2088, 4, 23),
                gregorian(2088, 5, 22),
                gregorian(2088, 6, 20),
                gregorian(2088, 7, 20),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1512,
            [
                gregorian(2088, 7, 20),
                gregorian(2088, 8, 18),
                gregorian(2088, 9, 17),
                gregorian(2088, 10, 16),
                gregorian(2088, 11, 15),
                gregorian(2088, 12, 14),
                gregorian(2089, 1, 13),
                gregorian(2089, 2, 12),
                gregorian(2089, 3, 14),
                gregorian(2089, 4, 12),
                gregorian(2089, 5, 12),
                gregorian(2089, 6, 10),
                gregorian(2089, 7, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1513,
            [
                gregorian(2089, 7, 10),
                gregorian(2089, 8, 8),
                gregorian(2089, 9, 6),
                gregorian(2089, 10, 5),
                gregorian(2089, 11, 4),
                gregorian(2089, 12, 3),
                gregorian(2090, 1, 2),
                gregorian(2090, 2, 1),
                gregorian(2090, 3, 3),
                gregorian(2090, 4, 1),
                gregorian(2090, 5, 1),
                gregorian(2090, 5, 31),
                gregorian(2090, 6, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1514,
            [
                gregorian(2090, 6, 29),
                gregorian(2090, 7, 29),
                gregorian(2090, 8, 27),
                gregorian(2090, 9, 25),
                gregorian(2090, 10, 24),
                gregorian(2090, 11, 23),
                gregorian(2090, 12, 22),
                gregorian(2091, 1, 21),
                gregorian(2091, 2, 20),
                gregorian(2091, 3, 21),
                gregorian(2091, 4, 20),
                gregorian(2091, 5, 20),
                gregorian(2091, 6, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1515,
            [
                gregorian(2091, 6, 19),
                gregorian(2091, 7, 18),
                gregorian(2091, 8, 16),
                gregorian(2091, 9, 15),
                gregorian(2091, 10, 14),
                gregorian(2091, 11, 12),
                gregorian(2091, 12, 12),
                gregorian(2092, 1, 10),
                gregorian(2092, 2, 9),
                gregorian(2092, 3, 10),
                gregorian(2092, 4, 8),
                gregorian(2092, 5, 8),
                gregorian(2092, 6, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1516,
            [
                gregorian(2092, 6, 7),
                gregorian(2092, 7, 6),
                gregorian(2092, 8, 5),
                gregorian(2092, 9, 3),
                gregorian(2092, 10, 3),
                gregorian(2092, 11, 1),
                gregorian(2092, 11, 30),
                gregorian(2092, 12, 30),
                gregorian(2093, 1, 28),
                gregorian(2093, 2, 27),
                gregorian(2093, 3, 28),
                gregorian(2093, 4, 27),
                gregorian(2093, 5, 27),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1517,
            [
                gregorian(2093, 5, 27),
                gregorian(2093, 6, 25),
                gregorian(2093, 7, 25),
                gregorian(2093, 8, 23),
                gregorian(2093, 9, 22),
                gregorian(2093, 10, 21),
                gregorian(2093, 11, 20),
                gregorian(2093, 12, 20),
                gregorian(2094, 1, 18),
                gregorian(2094, 2, 16),
                gregorian(2094, 3, 18),
                gregorian(2094, 4, 16),
                gregorian(2094, 5, 16),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1518,
            [
                gregorian(2094, 5, 16),
                gregorian(2094, 6, 14),
                gregorian(2094, 7, 14),
                gregorian(2094, 8, 12),
                gregorian(2094, 9, 11),
                gregorian(2094, 10, 11),
                gregorian(2094, 11, 9),
                gregorian(2094, 12, 9),
                gregorian(2095, 1, 8),
                gregorian(2095, 2, 6),
                gregorian(2095, 3, 8),
                gregorian(2095, 4, 6),
                gregorian(2095, 5, 5),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1519,
            [
                gregorian(2095, 5, 5),
                gregorian(2095, 6, 4),
                gregorian(2095, 7, 3),
                gregorian(2095, 8, 1),
                gregorian(2095, 8, 31),
                gregorian(2095, 9, 30),
                gregorian(2095, 10, 30),
                gregorian(2095, 11, 28),
                gregorian(2095, 12, 28),
                gregorian(2096, 1, 27),
                gregorian(2096, 2, 25),
                gregorian(2096, 3, 26),
                gregorian(2096, 4, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1520,
            [
                gregorian(2096, 4, 24),
                gregorian(2096, 5, 23),
                gregorian(2096, 6, 22),
                gregorian(2096, 7, 21),
                gregorian(2096, 8, 19),
                gregorian(2096, 9, 18),
                gregorian(2096, 10, 18),
                gregorian(2096, 11, 17),
                gregorian(2096, 12, 16),
                gregorian(2097, 1, 15),
                gregorian(2097, 2, 14),
                gregorian(2097, 3, 15),
                gregorian(2097, 4, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1521,
            [
                gregorian(2097, 4, 14),
                gregorian(2097, 5, 13),
                gregorian(2097, 6, 11),
                gregorian(2097, 7, 10),
                gregorian(2097, 8, 9),
                gregorian(2097, 9, 7),
                gregorian(2097, 10, 7),
                gregorian(2097, 11, 6),
                gregorian(2097, 12, 5),
                gregorian(2098, 1, 4),
                gregorian(2098, 2, 3),
                gregorian(2098, 3, 4),
                gregorian(2098, 4, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1522,
            [
                gregorian(2098, 4, 3),
                gregorian(2098, 5, 3),
                gregorian(2098, 6, 1),
                gregorian(2098, 6, 30),
                gregorian(2098, 7, 29),
                gregorian(2098, 8, 28),
                gregorian(2098, 9, 26),
                gregorian(2098, 10, 26),
                gregorian(2098, 11, 25),
                gregorian(2098, 12, 24),
                gregorian(2099, 1, 23),
                gregorian(2099, 2, 22),
                gregorian(2099, 3, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1523,
            [
                gregorian(2099, 3, 23),
                gregorian(2099, 4, 22),
                gregorian(2099, 5, 21),
                gregorian(2099, 6, 20),
                gregorian(2099, 7, 19),
                gregorian(2099, 8, 18),
                gregorian(2099, 9, 16),
                gregorian(2099, 10, 16),
                gregorian(2099, 11, 14),
                gregorian(2099, 12, 13),
                gregorian(2100, 1, 12),
                gregorian(2100, 2, 11),
                gregorian(2100, 3, 12),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1524,
            [
                gregorian(2100, 3, 12),
                gregorian(2100, 4, 11),
                gregorian(2100, 5, 11),
                gregorian(2100, 6, 9),
                gregorian(2100, 7, 9),
                gregorian(2100, 8, 7),
                gregorian(2100, 9, 6),
                gregorian(2100, 10, 5),
                gregorian(2100, 11, 4),
                gregorian(2100, 12, 3),
                gregorian(2101, 1, 1),
                gregorian(2101, 1, 31),
                gregorian(2101, 3, 1),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1525,
            [
                gregorian(2101, 3, 1),
                gregorian(2101, 3, 31),
                gregorian(2101, 4, 30),
                gregorian(2101, 5, 29),
                gregorian(2101, 6, 28),
                gregorian(2101, 7, 28),
                gregorian(2101, 8, 26),
                gregorian(2101, 9, 25),
                gregorian(2101, 10, 24),
                gregorian(2101, 11, 23),
                gregorian(2101, 12, 22),
                gregorian(2102, 1, 20),
                gregorian(2102, 2, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1526,
            [
                gregorian(2102, 2, 19),
                gregorian(2102, 3, 20),
                gregorian(2102, 4, 19),
                gregorian(2102, 5, 18),
                gregorian(2102, 6, 17),
                gregorian(2102, 7, 17),
                gregorian(2102, 8, 16),
                gregorian(2102, 9, 14),
                gregorian(2102, 10, 14),
                gregorian(2102, 11, 12),
                gregorian(2102, 12, 12),
                gregorian(2103, 1, 10),
                gregorian(2103, 2, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1527,
            [
                gregorian(2103, 2, 8),
                gregorian(2103, 3, 10),
                gregorian(2103, 4, 8),
                gregorian(2103, 5, 8),
                gregorian(2103, 6, 6),
                gregorian(2103, 7, 6),
                gregorian(2103, 8, 5),
                gregorian(2103, 9, 3),
                gregorian(2103, 10, 3),
                gregorian(2103, 11, 2),
                gregorian(2103, 12, 1),
                gregorian(2103, 12, 31),
                gregorian(2104, 1, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1528,
            [
                gregorian(2104, 1, 29),
                gregorian(2104, 2, 28),
                gregorian(2104, 3, 28),
                gregorian(2104, 4, 26),
                gregorian(2104, 5, 26),
                gregorian(2104, 6, 24),
                gregorian(2104, 7, 24),
                gregorian(2104, 8, 22),
                gregorian(2104, 9, 21),
                gregorian(2104, 10, 21),
                gregorian(2104, 11, 19),
                gregorian(2104, 12, 19),
                gregorian(2105, 1, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1529,
            [
                gregorian(2105, 1, 18),
                gregorian(2105, 2, 16),
                gregorian(2105, 3, 18),
                gregorian(2105, 4, 16),
                gregorian(2105, 5, 15),
                gregorian(2105, 6, 14),
                gregorian(2105, 7, 13),
                gregorian(2105, 8, 12),
                gregorian(2105, 9, 10),
                gregorian(2105, 10, 10),
                gregorian(2105, 11, 8),
                gregorian(2105, 12, 8),
                gregorian(2106, 1, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1530,
            [
                gregorian(2106, 1, 7),
                gregorian(2106, 2, 5),
                gregorian(2106, 3, 7),
                gregorian(2106, 4, 6),
                gregorian(2106, 5, 5),
                gregorian(2106, 6, 3),
                gregorian(2106, 7, 3),
                gregorian(2106, 8, 1),
                gregorian(2106, 8, 31),
                gregorian(2106, 9, 29),
                gregorian(2106, 10, 28),
                gregorian(2106, 11, 27),
                gregorian(2106, 12, 27),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1531,
            [
                gregorian(2106, 12, 27),
                gregorian(2107, 1, 25),
                gregorian(2107, 2, 24),
                gregorian(2107, 3, 26),
                gregorian(2107, 4, 25),
                gregorian(2107, 5, 24),
                gregorian(2107, 6, 22),
                gregorian(2107, 7, 22),
                gregorian(2107, 8, 20),
                gregorian(2107, 9, 19),
                gregorian(2107, 10, 18),
                gregorian(2107, 11, 16),
                gregorian(2107, 12, 16),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1532,
            [
                gregorian(2107, 12, 16),
                gregorian(2108, 1, 14),
                gregorian(2108, 2, 13),
                gregorian(2108, 3, 14),
                gregorian(2108, 4, 13),
                gregorian(2108, 5, 12),
                gregorian(2108, 6, 11),
                gregorian(2108, 7, 11),
                gregorian(2108, 8, 9),
                gregorian(2108, 9, 7),
                gregorian(2108, 10, 6),
                gregorian(2108, 11, 5),
                gregorian(2108, 12, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1533,
            [
                gregorian(2108, 12, 4),
                gregorian(2109, 1, 3),
                gregorian(2109, 2, 1),
                gregorian(2109, 3, 3),
                gregorian(2109, 4, 2),
                gregorian(2109, 5, 2),
                gregorian(2109, 5, 31),
                gregorian(2109, 6, 30),
                gregorian(2109, 7, 29),
                gregorian(2109, 8, 28),
                gregorian(2109, 9, 26),
                gregorian(2109, 10, 25),
                gregorian(2109, 11, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1534,
            [
                gregorian(2109, 11, 24),
                gregorian(2109, 12, 23),
                gregorian(2110, 1, 22),
                gregorian(2110, 2, 20),
                gregorian(2110, 3, 22),
                gregorian(2110, 4, 21),
                gregorian(2110, 5, 20),
                gregorian(2110, 6, 19),
                gregorian(2110, 7, 19),
                gregorian(2110, 8, 17),
                gregorian(2110, 9, 15),
                gregorian(2110, 10, 15),
                gregorian(2110, 11, 13),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1535,
            [
                gregorian(2110, 11, 13),
                gregorian(2110, 12, 13),
                gregorian(2111, 1, 11),
                gregorian(2111, 2, 10),
                gregorian(2111, 3, 11),
                gregorian(2111, 4, 10),
                gregorian(2111, 5, 9),
                gregorian(2111, 6, 8),
                gregorian(2111, 7, 8),
                gregorian(2111, 8, 6),
                gregorian(2111, 9, 5),
                gregorian(2111, 10, 4),
                gregorian(2111, 11, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1536,
            [
                gregorian(2111, 11, 3),
                gregorian(2111, 12, 2),
                gregorian(2112, 1, 1),
                gregorian(2112, 1, 30),
                gregorian(2112, 2, 29),
                gregorian(2112, 3, 29),
                gregorian(2112, 4, 28),
                gregorian(2112, 5, 27),
                gregorian(2112, 6, 26),
                gregorian(2112, 7, 25),
                gregorian(2112, 8, 24),
                gregorian(2112, 9, 22),
                gregorian(2112, 10, 22),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1537,
            [
                gregorian(2112, 10, 22),
                gregorian(2112, 11, 21),
                gregorian(2112, 12, 20),
                gregorian(2113, 1, 19),
                gregorian(2113, 2, 18),
                gregorian(2113, 3, 19),
                gregorian(2113, 4, 17),
                gregorian(2113, 5, 17),
                gregorian(2113, 6, 15),
                gregorian(2113, 7, 14),
                gregorian(2113, 8, 13),
                gregorian(2113, 9, 11),
                gregorian(2113, 10, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1538,
            [
                gregorian(2113, 10, 11),
                gregorian(2113, 11, 10),
                gregorian(2113, 12, 10),
                gregorian(2114, 1, 8),
                gregorian(2114, 2, 7),
                gregorian(2114, 3, 9),
                gregorian(2114, 4, 7),
                gregorian(2114, 5, 6),
                gregorian(2114, 6, 5),
                gregorian(2114, 7, 4),
                gregorian(2114, 8, 2),
                gregorian(2114, 9, 1),
                gregorian(2114, 9, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1539,
            [
                gregorian(2114, 9, 30),
                gregorian(2114, 10, 30),
                gregorian(2114, 11, 29),
                gregorian(2114, 12, 29),
                gregorian(2115, 1, 27),
                gregorian(2115, 2, 26),
                gregorian(2115, 3, 28),
                gregorian(2115, 4, 26),
                gregorian(2115, 5, 25),
                gregorian(2115, 6, 24),
                gregorian(2115, 7, 23),
                gregorian(2115, 8, 21),
                gregorian(2115, 9, 20),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1540,
            [
                gregorian(2115, 9, 20),
                gregorian(2115, 10, 19),
                gregorian(2115, 11, 18),
                gregorian(2115, 12, 18),
                gregorian(2116, 1, 16),
                gregorian(2116, 2, 15),
                gregorian(2116, 3, 16),
                gregorian(2116, 4, 14),
                gregorian(2116, 5, 14),
                gregorian(2116, 6, 12),
                gregorian(2116, 7, 11),
                gregorian(2116, 8, 10),
                gregorian(2116, 9, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1541,
            [
                gregorian(2116, 9, 8),
                gregorian(2116, 10, 8),
                gregorian(2116, 11, 6),
                gregorian(2116, 12, 6),
                gregorian(2117, 1, 4),
                gregorian(2117, 2, 3),
                gregorian(2117, 3, 5),
                gregorian(2117, 4, 4),
                gregorian(2117, 5, 3),
                gregorian(2117, 6, 2),
                gregorian(2117, 7, 1),
                gregorian(2117, 7, 30),
                gregorian(2117, 8, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1542,
            [
                gregorian(2117, 8, 29),
                gregorian(2117, 9, 27),
                gregorian(2117, 10, 27),
                gregorian(2117, 11, 25),
                gregorian(2117, 12, 25),
                gregorian(2118, 1, 23),
                gregorian(2118, 2, 22),
                gregorian(2118, 3, 24),
                gregorian(2118, 4, 22),
                gregorian(2118, 5, 22),
                gregorian(2118, 6, 20),
                gregorian(2118, 7, 20),
                gregorian(2118, 8, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1543,
            [
                gregorian(2118, 8, 19),
                gregorian(2118, 9, 17),
                gregorian(2118, 10, 17),
                gregorian(2118, 11, 15),
                gregorian(2118, 12, 14),
                gregorian(2119, 1, 13),
                gregorian(2119, 2, 11),
                gregorian(2119, 3, 13),
                gregorian(2119, 4, 11),
                gregorian(2119, 5, 11),
                gregorian(2119, 6, 9),
                gregorian(2119, 7, 9),
                gregorian(2119, 8, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1544,
            [
                gregorian(2119, 8, 8),
                gregorian(2119, 9, 7),
                gregorian(2119, 10, 6),
                gregorian(2119, 11, 5),
                gregorian(2119, 12, 4),
                gregorian(2120, 1, 2),
                gregorian(2120, 2, 1),
                gregorian(2120, 3, 1),
                gregorian(2120, 3, 31),
                gregorian(2120, 4, 29),
                gregorian(2120, 5, 29),
                gregorian(2120, 6, 27),
                gregorian(2120, 7, 27),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1545,
            [
                gregorian(2120, 7, 27),
                gregorian(2120, 8, 26),
                gregorian(2120, 9, 25),
                gregorian(2120, 10, 24),
                gregorian(2120, 11, 23),
                gregorian(2120, 12, 22),
                gregorian(2121, 1, 20),
                gregorian(2121, 2, 19),
                gregorian(2121, 3, 20),
                gregorian(2121, 4, 19),
                gregorian(2121, 5, 18),
                gregorian(2121, 6, 16),
                gregorian(2121, 7, 16),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1546,
            [
                gregorian(2121, 7, 16),
                gregorian(2121, 8, 15),
                gregorian(2121, 9, 14),
                gregorian(2121, 10, 13),
                gregorian(2121, 11, 12),
                gregorian(2121, 12, 11),
                gregorian(2122, 1, 10),
                gregorian(2122, 2, 8),
                gregorian(2122, 3, 10),
                gregorian(2122, 4, 8),
                gregorian(2122, 5, 8),
                gregorian(2122, 6, 6),
                gregorian(2122, 7, 5),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1547,
            [
                gregorian(2122, 7, 5),
                gregorian(2122, 8, 4),
                gregorian(2122, 9, 3),
                gregorian(2122, 10, 2),
                gregorian(2122, 11, 1),
                gregorian(2122, 12, 1),
                gregorian(2122, 12, 30),
                gregorian(2123, 1, 29),
                gregorian(2123, 2, 27),
                gregorian(2123, 3, 29),
                gregorian(2123, 4, 27),
                gregorian(2123, 5, 27),
                gregorian(2123, 6, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1548,
            [
                gregorian(2123, 6, 25),
                gregorian(2123, 7, 25),
                gregorian(2123, 8, 23),
                gregorian(2123, 9, 21),
                gregorian(2123, 10, 21),
                gregorian(2123, 11, 20),
                gregorian(2123, 12, 19),
                gregorian(2124, 1, 18),
                gregorian(2124, 2, 17),
                gregorian(2124, 3, 17),
                gregorian(2124, 4, 16),
                gregorian(2124, 5, 15),
                gregorian(2124, 6, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1549,
            [
                gregorian(2124, 6, 14),
                gregorian(2124, 7, 13),
                gregorian(2124, 8, 12),
                gregorian(2124, 9, 10),
                gregorian(2124, 10, 9),
                gregorian(2124, 11, 8),
                gregorian(2124, 12, 7),
                gregorian(2125, 1, 6),
                gregorian(2125, 2, 5),
                gregorian(2125, 3, 7),
                gregorian(2125, 4, 5),
                gregorian(2125, 5, 5),
                gregorian(2125, 6, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1550,
            [
                gregorian(2125, 6, 3),
                gregorian(2125, 7, 3),
                gregorian(2125, 8, 1),
                gregorian(2125, 8, 31),
                gregorian(2125, 9, 29),
                gregorian(2125, 10, 28),
                gregorian(2125, 11, 26),
                gregorian(2125, 12, 26),
                gregorian(2126, 1, 25),
                gregorian(2126, 2, 24),
                gregorian(2126, 3, 25),
                gregorian(2126, 4, 24),
                gregorian(2126, 5, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1551,
            [
                gregorian(2126, 5, 24),
                gregorian(2126, 6, 22),
                gregorian(2126, 7, 22),
                gregorian(2126, 8, 20),
                gregorian(2126, 9, 18),
                gregorian(2126, 10, 18),
                gregorian(2126, 11, 16),
                gregorian(2126, 12, 15),
                gregorian(2127, 1, 14),
                gregorian(2127, 2, 13),
                gregorian(2127, 3, 14),
                gregorian(2127, 4, 13),
                gregorian(2127, 5, 13),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1552,
            [
                gregorian(2127, 5, 13),
                gregorian(2127, 6, 12),
                gregorian(2127, 7, 11),
                gregorian(2127, 8, 10),
                gregorian(2127, 9, 8),
                gregorian(2127, 10, 7),
                gregorian(2127, 11, 6),
                gregorian(2127, 12, 5),
                gregorian(2128, 1, 3),
                gregorian(2128, 2, 2),
                gregorian(2128, 3, 3),
                gregorian(2128, 4, 1),
                gregorian(2128, 5, 1),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1553,
            [
                gregorian(2128, 5, 1),
                gregorian(2128, 5, 31),
                gregorian(2128, 6, 29),
                gregorian(2128, 7, 29),
                gregorian(2128, 8, 27),
                gregorian(2128, 9, 26),
                gregorian(2128, 10, 25),
                gregorian(2128, 11, 24),
                gregorian(2128, 12, 23),
                gregorian(2129, 1, 22),
                gregorian(2129, 2, 20),
                gregorian(2129, 3, 22),
                gregorian(2129, 4, 20),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1554,
            [
                gregorian(2129, 4, 20),
                gregorian(2129, 5, 20),
                gregorian(2129, 6, 18),
                gregorian(2129, 7, 18),
                gregorian(2129, 8, 16),
                gregorian(2129, 9, 15),
                gregorian(2129, 10, 15),
                gregorian(2129, 11, 13),
                gregorian(2129, 12, 13),
                gregorian(2130, 1, 11),
                gregorian(2130, 2, 10),
                gregorian(2130, 3, 11),
                gregorian(2130, 4, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1555,
            [
                gregorian(2130, 4, 10),
                gregorian(2130, 5, 9),
                gregorian(2130, 6, 7),
                gregorian(2130, 7, 7),
                gregorian(2130, 8, 5),
                gregorian(2130, 9, 4),
                gregorian(2130, 10, 4),
                gregorian(2130, 11, 2),
                gregorian(2130, 12, 2),
                gregorian(2131, 1, 1),
                gregorian(2131, 1, 30),
                gregorian(2131, 3, 1),
                gregorian(2131, 3, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1556,
            [
                gregorian(2131, 3, 30),
                gregorian(2131, 4, 29),
                gregorian(2131, 5, 28),
                gregorian(2131, 6, 26),
                gregorian(2131, 7, 26),
                gregorian(2131, 8, 24),
                gregorian(2131, 9, 23),
                gregorian(2131, 10, 22),
                gregorian(2131, 11, 21),
                gregorian(2131, 12, 21),
                gregorian(2132, 1, 20),
                gregorian(2132, 2, 18),
                gregorian(2132, 3, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1557,
            [
                gregorian(2132, 3, 19),
                gregorian(2132, 4, 17),
                gregorian(2132, 5, 17),
                gregorian(2132, 6, 15),
                gregorian(2132, 7, 14),
                gregorian(2132, 8, 12),
                gregorian(2132, 9, 11),
                gregorian(2132, 10, 10),
                gregorian(2132, 11, 9),
                gregorian(2132, 12, 9),
                gregorian(2133, 1, 8),
                gregorian(2133, 2, 7),
                gregorian(2133, 3, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1558,
            [
                gregorian(2133, 3, 8),
                gregorian(2133, 4, 7),
                gregorian(2133, 5, 6),
                gregorian(2133, 6, 5),
                gregorian(2133, 7, 4),
                gregorian(2133, 8, 2),
                gregorian(2133, 8, 31),
                gregorian(2133, 9, 30),
                gregorian(2133, 10, 29),
                gregorian(2133, 11, 28),
                gregorian(2133, 12, 28),
                gregorian(2134, 1, 27),
                gregorian(2134, 2, 25),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1559,
            [
                gregorian(2134, 2, 25),
                gregorian(2134, 3, 27),
                gregorian(2134, 4, 26),
                gregorian(2134, 5, 25),
                gregorian(2134, 6, 23),
                gregorian(2134, 7, 23),
                gregorian(2134, 8, 21),
                gregorian(2134, 9, 19),
                gregorian(2134, 10, 19),
                gregorian(2134, 11, 18),
                gregorian(2134, 12, 17),
                gregorian(2135, 1, 16),
                gregorian(2135, 2, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1560,
            [
                gregorian(2135, 2, 14),
                gregorian(2135, 3, 16),
                gregorian(2135, 4, 15),
                gregorian(2135, 5, 14),
                gregorian(2135, 6, 13),
                gregorian(2135, 7, 12),
                gregorian(2135, 8, 11),
                gregorian(2135, 9, 9),
                gregorian(2135, 10, 9),
                gregorian(2135, 11, 7),
                gregorian(2135, 12, 7),
                gregorian(2136, 1, 5),
                gregorian(2136, 2, 4),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1561,
            [
                gregorian(2136, 2, 4),
                gregorian(2136, 3, 4),
                gregorian(2136, 4, 3),
                gregorian(2136, 5, 3),
                gregorian(2136, 6, 1),
                gregorian(2136, 7, 1),
                gregorian(2136, 7, 30),
                gregorian(2136, 8, 29),
                gregorian(2136, 9, 28),
                gregorian(2136, 10, 27),
                gregorian(2136, 11, 25),
                gregorian(2136, 12, 25),
                gregorian(2137, 1, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1562,
            [
                gregorian(2137, 1, 23),
                gregorian(2137, 2, 21),
                gregorian(2137, 3, 23),
                gregorian(2137, 4, 22),
                gregorian(2137, 5, 21),
                gregorian(2137, 6, 20),
                gregorian(2137, 7, 19),
                gregorian(2137, 8, 18),
                gregorian(2137, 9, 17),
                gregorian(2137, 10, 17),
                gregorian(2137, 11, 15),
                gregorian(2137, 12, 14),
                gregorian(2138, 1, 13),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1563,
            [
                gregorian(2138, 1, 13),
                gregorian(2138, 2, 11),
                gregorian(2138, 3, 13),
                gregorian(2138, 4, 11),
                gregorian(2138, 5, 10),
                gregorian(2138, 6, 9),
                gregorian(2138, 7, 8),
                gregorian(2138, 8, 7),
                gregorian(2138, 9, 6),
                gregorian(2138, 10, 6),
                gregorian(2138, 11, 4),
                gregorian(2138, 12, 4),
                gregorian(2139, 1, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1564,
            [
                gregorian(2139, 1, 2),
                gregorian(2139, 2, 1),
                gregorian(2139, 3, 2),
                gregorian(2139, 4, 1),
                gregorian(2139, 4, 30),
                gregorian(2139, 5, 29),
                gregorian(2139, 6, 28),
                gregorian(2139, 7, 27),
                gregorian(2139, 8, 26),
                gregorian(2139, 9, 25),
                gregorian(2139, 10, 25),
                gregorian(2139, 11, 23),
                gregorian(2139, 12, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1565,
            [
                gregorian(2139, 12, 23),
                gregorian(2140, 1, 21),
                gregorian(2140, 2, 20),
                gregorian(2140, 3, 20),
                gregorian(2140, 4, 19),
                gregorian(2140, 5, 18),
                gregorian(2140, 6, 16),
                gregorian(2140, 7, 16),
                gregorian(2140, 8, 14),
                gregorian(2140, 9, 13),
                gregorian(2140, 10, 13),
                gregorian(2140, 11, 11),
                gregorian(2140, 12, 11),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1566,
            [
                gregorian(2140, 12, 11),
                gregorian(2141, 1, 10),
                gregorian(2141, 2, 8),
                gregorian(2141, 3, 10),
                gregorian(2141, 4, 8),
                gregorian(2141, 5, 8),
                gregorian(2141, 6, 6),
                gregorian(2141, 7, 5),
                gregorian(2141, 8, 4),
                gregorian(2141, 9, 2),
                gregorian(2141, 10, 2),
                gregorian(2141, 10, 31),
                gregorian(2141, 11, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1567,
            [
                gregorian(2141, 11, 30),
                gregorian(2141, 12, 30),
                gregorian(2142, 1, 28),
                gregorian(2142, 2, 27),
                gregorian(2142, 3, 29),
                gregorian(2142, 4, 27),
                gregorian(2142, 5, 27),
                gregorian(2142, 6, 25),
                gregorian(2142, 7, 25),
                gregorian(2142, 8, 23),
                gregorian(2142, 9, 21),
                gregorian(2142, 10, 21),
                gregorian(2142, 11, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1568,
            [
                gregorian(2142, 11, 19),
                gregorian(2142, 12, 19),
                gregorian(2143, 1, 17),
                gregorian(2143, 2, 16),
                gregorian(2143, 3, 18),
                gregorian(2143, 4, 17),
                gregorian(2143, 5, 16),
                gregorian(2143, 6, 15),
                gregorian(2143, 7, 14),
                gregorian(2143, 8, 13),
                gregorian(2143, 9, 11),
                gregorian(2143, 10, 10),
                gregorian(2143, 11, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1569,
            [
                gregorian(2143, 11, 8),
                gregorian(2143, 12, 8),
                gregorian(2144, 1, 6),
                gregorian(2144, 2, 5),
                gregorian(2144, 3, 6),
                gregorian(2144, 4, 5),
                gregorian(2144, 5, 4),
                gregorian(2144, 6, 3),
                gregorian(2144, 7, 3),
                gregorian(2144, 8, 1),
                gregorian(2144, 8, 31),
                gregorian(2144, 9, 29),
                gregorian(2144, 10, 28),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1570,
            [
                gregorian(2144, 10, 28),
                gregorian(2144, 11, 26),
                gregorian(2144, 12, 26),
                gregorian(2145, 1, 24),
                gregorian(2145, 2, 23),
                gregorian(2145, 3, 25),
                gregorian(2145, 4, 23),
                gregorian(2145, 5, 23),
                gregorian(2145, 6, 22),
                gregorian(2145, 7, 22),
                gregorian(2145, 8, 20),
                gregorian(2145, 9, 18),
                gregorian(2145, 10, 18),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1571,
            [
                gregorian(2145, 10, 18),
                gregorian(2145, 11, 16),
                gregorian(2145, 12, 15),
                gregorian(2146, 1, 14),
                gregorian(2146, 2, 12),
                gregorian(2146, 3, 14),
                gregorian(2146, 4, 13),
                gregorian(2146, 5, 12),
                gregorian(2146, 6, 11),
                gregorian(2146, 7, 11),
                gregorian(2146, 8, 9),
                gregorian(2146, 9, 8),
                gregorian(2146, 10, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1572,
            [
                gregorian(2146, 10, 7),
                gregorian(2146, 11, 6),
                gregorian(2146, 12, 5),
                gregorian(2147, 1, 3),
                gregorian(2147, 2, 2),
                gregorian(2147, 3, 3),
                gregorian(2147, 4, 2),
                gregorian(2147, 5, 1),
                gregorian(2147, 5, 31),
                gregorian(2147, 6, 30),
                gregorian(2147, 7, 29),
                gregorian(2147, 8, 28),
                gregorian(2147, 9, 26),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1573,
            [
                gregorian(2147, 9, 26),
                gregorian(2147, 10, 26),
                gregorian(2147, 11, 24),
                gregorian(2147, 12, 24),
                gregorian(2148, 1, 23),
                gregorian(2148, 2, 21),
                gregorian(2148, 3, 22),
                gregorian(2148, 4, 20),
                gregorian(2148, 5, 19),
                gregorian(2148, 6, 18),
                gregorian(2148, 7, 17),
                gregorian(2148, 8, 16),
                gregorian(2148, 9, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1574,
            [
                gregorian(2148, 9, 14),
                gregorian(2148, 10, 14),
                gregorian(2148, 11, 13),
                gregorian(2148, 12, 12),
                gregorian(2149, 1, 11),
                gregorian(2149, 2, 10),
                gregorian(2149, 3, 11),
                gregorian(2149, 4, 10),
                gregorian(2149, 5, 9),
                gregorian(2149, 6, 7),
                gregorian(2149, 7, 7),
                gregorian(2149, 8, 5),
                gregorian(2149, 9, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1575,
            [
                gregorian(2149, 9, 3),
                gregorian(2149, 10, 3),
                gregorian(2149, 11, 2),
                gregorian(2149, 12, 2),
                gregorian(2149, 12, 31),
                gregorian(2150, 1, 30),
                gregorian(2150, 3, 1),
                gregorian(2150, 3, 30),
                gregorian(2150, 4, 29),
                gregorian(2150, 5, 28),
                gregorian(2150, 6, 26),
                gregorian(2150, 7, 25),
                gregorian(2150, 8, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1576,
            [
                gregorian(2150, 8, 24),
                gregorian(2150, 9, 22),
                gregorian(2150, 10, 22),
                gregorian(2150, 11, 21),
                gregorian(2150, 12, 20),
                gregorian(2151, 1, 19),
                gregorian(2151, 2, 18),
                gregorian(2151, 3, 20),
                gregorian(2151, 4, 18),
                gregorian(2151, 5, 18),
                gregorian(2151, 6, 16),
                gregorian(2151, 7, 15),
                gregorian(2151, 8, 13),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1577,
            [
                gregorian(2151, 8, 13),
                gregorian(2151, 9, 12),
                gregorian(2151, 10, 11),
                gregorian(2151, 11, 10),
                gregorian(2151, 12, 10),
                gregorian(2152, 1, 8),
                gregorian(2152, 2, 7),
                gregorian(2152, 3, 8),
                gregorian(2152, 4, 6),
                gregorian(2152, 5, 6),
                gregorian(2152, 6, 4),
                gregorian(2152, 7, 4),
                gregorian(2152, 8, 2),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1578,
            [
                gregorian(2152, 8, 2),
                gregorian(2152, 8, 31),
                gregorian(2152, 9, 30),
                gregorian(2152, 10, 29),
                gregorian(2152, 11, 28),
                gregorian(2152, 12, 27),
                gregorian(2153, 1, 26),
                gregorian(2153, 2, 25),
                gregorian(2153, 3, 26),
                gregorian(2153, 4, 25),
                gregorian(2153, 5, 25),
                gregorian(2153, 6, 23),
                gregorian(2153, 7, 23),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1579,
            [
                gregorian(2153, 7, 23),
                gregorian(2153, 8, 21),
                gregorian(2153, 9, 20),
                gregorian(2153, 10, 19),
                gregorian(2153, 11, 18),
                gregorian(2153, 12, 17),
                gregorian(2154, 1, 15),
                gregorian(2154, 2, 14),
                gregorian(2154, 3, 16),
                gregorian(2154, 4, 14),
                gregorian(2154, 5, 14),
                gregorian(2154, 6, 12),
                gregorian(2154, 7, 12),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1580,
            [
                gregorian(2154, 7, 12),
                gregorian(2154, 8, 10),
                gregorian(2154, 9, 9),
                gregorian(2154, 10, 9),
                gregorian(2154, 11, 7),
                gregorian(2154, 12, 7),
                gregorian(2155, 1, 5),
                gregorian(2155, 2, 3),
                gregorian(2155, 3, 5),
                gregorian(2155, 4, 3),
                gregorian(2155, 5, 3),
                gregorian(2155, 6, 1),
                gregorian(2155, 7, 1),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1581,
            [
                gregorian(2155, 7, 1),
                gregorian(2155, 7, 31),
                gregorian(2155, 8, 30),
                gregorian(2155, 9, 28),
                gregorian(2155, 10, 28),
                gregorian(2155, 11, 26),
                gregorian(2155, 12, 26),
                gregorian(2156, 1, 24),
                gregorian(2156, 2, 22),
                gregorian(2156, 3, 23),
                gregorian(2156, 4, 21),
                gregorian(2156, 5, 21),
                gregorian(2156, 6, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1582,
            [
                gregorian(2156, 6, 19),
                gregorian(2156, 7, 19),
                gregorian(2156, 8, 18),
                gregorian(2156, 9, 16),
                gregorian(2156, 10, 16),
                gregorian(2156, 11, 15),
                gregorian(2156, 12, 14),
                gregorian(2157, 1, 13),
                gregorian(2157, 2, 11),
                gregorian(2157, 3, 13),
                gregorian(2157, 4, 11),
                gregorian(2157, 5, 10),
                gregorian(2157, 6, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1583,
            [
                gregorian(2157, 6, 8),
                gregorian(2157, 7, 8),
                gregorian(2157, 8, 7),
                gregorian(2157, 9, 5),
                gregorian(2157, 10, 5),
                gregorian(2157, 11, 4),
                gregorian(2157, 12, 4),
                gregorian(2158, 1, 2),
                gregorian(2158, 2, 1),
                gregorian(2158, 3, 2),
                gregorian(2158, 4, 1),
                gregorian(2158, 4, 30),
                gregorian(2158, 5, 29),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1584,
            [
                gregorian(2158, 5, 29),
                gregorian(2158, 6, 27),
                gregorian(2158, 7, 27),
                gregorian(2158, 8, 26),
                gregorian(2158, 9, 24),
                gregorian(2158, 10, 24),
                gregorian(2158, 11, 23),
                gregorian(2158, 12, 22),
                gregorian(2159, 1, 21),
                gregorian(2159, 2, 20),
                gregorian(2159, 3, 21),
                gregorian(2159, 4, 20),
                gregorian(2159, 5, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1585,
            [
                gregorian(2159, 5, 19),
                gregorian(2159, 6, 17),
                gregorian(2159, 7, 17),
                gregorian(2159, 8, 15),
                gregorian(2159, 9, 14),
                gregorian(2159, 10, 13),
                gregorian(2159, 11, 12),
                gregorian(2159, 12, 11),
                gregorian(2160, 1, 10),
                gregorian(2160, 2, 9),
                gregorian(2160, 3, 9),
                gregorian(2160, 4, 8),
                gregorian(2160, 5, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1586,
            [
                gregorian(2160, 5, 8),
                gregorian(2160, 6, 6),
                gregorian(2160, 7, 5),
                gregorian(2160, 8, 4),
                gregorian(2160, 9, 2),
                gregorian(2160, 10, 2),
                gregorian(2160, 10, 31),
                gregorian(2160, 11, 29),
                gregorian(2160, 12, 29),
                gregorian(2161, 1, 28),
                gregorian(2161, 2, 27),
                gregorian(2161, 3, 28),
                gregorian(2161, 4, 27),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1587,
            [
                gregorian(2161, 4, 27),
                gregorian(2161, 5, 26),
                gregorian(2161, 6, 25),
                gregorian(2161, 7, 25),
                gregorian(2161, 8, 23),
                gregorian(2161, 9, 21),
                gregorian(2161, 10, 20),
                gregorian(2161, 11, 19),
                gregorian(2161, 12, 18),
                gregorian(2162, 1, 17),
                gregorian(2162, 2, 15),
                gregorian(2162, 3, 17),
                gregorian(2162, 4, 16),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1588,
            [
                gregorian(2162, 4, 16),
                gregorian(2162, 5, 16),
                gregorian(2162, 6, 14),
                gregorian(2162, 7, 14),
                gregorian(2162, 8, 13),
                gregorian(2162, 9, 11),
                gregorian(2162, 10, 10),
                gregorian(2162, 11, 8),
                gregorian(2162, 12, 8),
                gregorian(2163, 1, 6),
                gregorian(2163, 2, 5),
                gregorian(2163, 3, 6),
                gregorian(2163, 4, 5),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1589,
            [
                gregorian(2163, 4, 5),
                gregorian(2163, 5, 5),
                gregorian(2163, 6, 3),
                gregorian(2163, 7, 3),
                gregorian(2163, 8, 2),
                gregorian(2163, 8, 31),
                gregorian(2163, 9, 30),
                gregorian(2163, 10, 29),
                gregorian(2163, 11, 27),
                gregorian(2163, 12, 27),
                gregorian(2164, 1, 25),
                gregorian(2164, 2, 24),
                gregorian(2164, 3, 24),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1590,
            [
                gregorian(2164, 3, 24),
                gregorian(2164, 4, 23),
                gregorian(2164, 5, 22),
                gregorian(2164, 6, 21),
                gregorian(2164, 7, 21),
                gregorian(2164, 8, 20),
                gregorian(2164, 9, 18),
                gregorian(2164, 10, 17),
                gregorian(2164, 11, 16),
                gregorian(2164, 12, 15),
                gregorian(2165, 1, 14),
                gregorian(2165, 2, 12),
                gregorian(2165, 3, 14),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1591,
            [
                gregorian(2165, 3, 14),
                gregorian(2165, 4, 12),
                gregorian(2165, 5, 12),
                gregorian(2165, 6, 10),
                gregorian(2165, 7, 10),
                gregorian(2165, 8, 9),
                gregorian(2165, 9, 7),
                gregorian(2165, 10, 7),
                gregorian(2165, 11, 5),
                gregorian(2165, 12, 5),
                gregorian(2166, 1, 3),
                gregorian(2166, 2, 2),
                gregorian(2166, 3, 3),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1592,
            [
                gregorian(2166, 3, 3),
                gregorian(2166, 4, 2),
                gregorian(2166, 5, 1),
                gregorian(2166, 5, 31),
                gregorian(2166, 6, 29),
                gregorian(2166, 7, 29),
                gregorian(2166, 8, 27),
                gregorian(2166, 9, 26),
                gregorian(2166, 10, 25),
                gregorian(2166, 11, 24),
                gregorian(2166, 12, 24),
                gregorian(2167, 1, 23),
                gregorian(2167, 2, 21),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1593,
            [
                gregorian(2167, 2, 21),
                gregorian(2167, 3, 23),
                gregorian(2167, 4, 21),
                gregorian(2167, 5, 20),
                gregorian(2167, 6, 19),
                gregorian(2167, 7, 18),
                gregorian(2167, 8, 16),
                gregorian(2167, 9, 15),
                gregorian(2167, 10, 14),
                gregorian(2167, 11, 13),
                gregorian(2167, 12, 13),
                gregorian(2168, 1, 12),
                gregorian(2168, 2, 10),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1594,
            [
                gregorian(2168, 2, 10),
                gregorian(2168, 3, 11),
                gregorian(2168, 4, 10),
                gregorian(2168, 5, 9),
                gregorian(2168, 6, 7),
                gregorian(2168, 7, 7),
                gregorian(2168, 8, 5),
                gregorian(2168, 9, 3),
                gregorian(2168, 10, 2),
                gregorian(2168, 11, 1),
                gregorian(2168, 12, 1),
                gregorian(2168, 12, 31),
                gregorian(2169, 1, 30),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1595,
            [
                gregorian(2169, 1, 30),
                gregorian(2169, 2, 28),
                gregorian(2169, 3, 30),
                gregorian(2169, 4, 28),
                gregorian(2169, 5, 28),
                gregorian(2169, 6, 26),
                gregorian(2169, 7, 25),
                gregorian(2169, 8, 24),
                gregorian(2169, 9, 22),
                gregorian(2169, 10, 21),
                gregorian(2169, 11, 20),
                gregorian(2169, 12, 20),
                gregorian(2170, 1, 19),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1596,
            [
                gregorian(2170, 1, 19),
                gregorian(2170, 2, 17),
                gregorian(2170, 3, 19),
                gregorian(2170, 4, 18),
                gregorian(2170, 5, 17),
                gregorian(2170, 6, 16),
                gregorian(2170, 7, 15),
                gregorian(2170, 8, 13),
                gregorian(2170, 9, 12),
                gregorian(2170, 10, 11),
                gregorian(2170, 11, 10),
                gregorian(2170, 12, 9),
                gregorian(2171, 1, 8),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1597,
            [
                gregorian(2171, 1, 8),
                gregorian(2171, 2, 6),
                gregorian(2171, 3, 8),
                gregorian(2171, 4, 7),
                gregorian(2171, 5, 6),
                gregorian(2171, 6, 5),
                gregorian(2171, 7, 4),
                gregorian(2171, 8, 3),
                gregorian(2171, 9, 1),
                gregorian(2171, 10, 1),
                gregorian(2171, 10, 30),
                gregorian(2171, 11, 29),
                gregorian(2171, 12, 28),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1598,
            [
                gregorian(2171, 12, 28),
                gregorian(2172, 1, 27),
                gregorian(2172, 2, 25),
                gregorian(2172, 3, 26),
                gregorian(2172, 4, 24),
                gregorian(2172, 5, 24),
                gregorian(2172, 6, 23),
                gregorian(2172, 7, 22),
                gregorian(2172, 8, 21),
                gregorian(2172, 9, 19),
                gregorian(2172, 10, 19),
                gregorian(2172, 11, 18),
                gregorian(2172, 12, 17),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1599,
            [
                gregorian(2172, 12, 17),
                gregorian(2173, 1, 15),
                gregorian(2173, 2, 14),
                gregorian(2173, 3, 15),
                gregorian(2173, 4, 14),
                gregorian(2173, 5, 13),
                gregorian(2173, 6, 12),
                gregorian(2173, 7, 11),
                gregorian(2173, 8, 10),
                gregorian(2173, 9, 9),
                gregorian(2173, 10, 9),
                gregorian(2173, 11, 7),
                gregorian(2173, 12, 7),
            ],
        )
        .unwrap()
        .packed,
        HijriYear::try_new(
            1600,
            [
                gregorian(2173, 12, 7),
                gregorian(2174, 1, 5),
                gregorian(2174, 2, 3),
                gregorian(2174, 3, 5),
                gregorian(2174, 4, 3),
                gregorian(2174, 5, 3),
                gregorian(2174, 6, 1),
                gregorian(2174, 6, 30),
                gregorian(2174, 7, 30),
                gregorian(2174, 8, 29),
                gregorian(2174, 9, 28),
                gregorian(2174, 10, 27),
                gregorian(2174, 11, 26),
            ],
        )
        .unwrap()
        .packed,
    ]
};

#[test]
fn test_icu4c_agreement() {
    use calendrical_calculations::islamic::ISLAMIC_EPOCH_FRIDAY;

    // From https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L87
    const ICU4C_ENCODED_MONTH_LENGTHS: [u16; 1601 - 1300] = [
        0x0AAA, 0x0D54, 0x0EC9, 0x06D4, 0x06EA, 0x036C, 0x0AAD, 0x0555, 0x06A9, 0x0792, 0x0BA9,
        0x05D4, 0x0ADA, 0x055C, 0x0D2D, 0x0695, 0x074A, 0x0B54, 0x0B6A, 0x05AD, 0x04AE, 0x0A4F,
        0x0517, 0x068B, 0x06A5, 0x0AD5, 0x02D6, 0x095B, 0x049D, 0x0A4D, 0x0D26, 0x0D95, 0x05AC,
        0x09B6, 0x02BA, 0x0A5B, 0x052B, 0x0A95, 0x06CA, 0x0AE9, 0x02F4, 0x0976, 0x02B6, 0x0956,
        0x0ACA, 0x0BA4, 0x0BD2, 0x05D9, 0x02DC, 0x096D, 0x054D, 0x0AA5, 0x0B52, 0x0BA5, 0x05B4,
        0x09B6, 0x0557, 0x0297, 0x054B, 0x06A3, 0x0752, 0x0B65, 0x056A, 0x0AAB, 0x052B, 0x0C95,
        0x0D4A, 0x0DA5, 0x05CA, 0x0AD6, 0x0957, 0x04AB, 0x094B, 0x0AA5, 0x0B52, 0x0B6A, 0x0575,
        0x0276, 0x08B7, 0x045B, 0x0555, 0x05A9, 0x05B4, 0x09DA, 0x04DD, 0x026E, 0x0936, 0x0AAA,
        0x0D54, 0x0DB2, 0x05D5, 0x02DA, 0x095B, 0x04AB, 0x0A55, 0x0B49, 0x0B64, 0x0B71, 0x05B4,
        0x0AB5, 0x0A55, 0x0D25, 0x0E92, 0x0EC9, 0x06D4, 0x0AE9, 0x096B, 0x04AB, 0x0A93, 0x0D49,
        0x0DA4, 0x0DB2, 0x0AB9, 0x04BA, 0x0A5B, 0x052B, 0x0A95, 0x0B2A, 0x0B55, 0x055C, 0x04BD,
        0x023D, 0x091D, 0x0A95, 0x0B4A, 0x0B5A, 0x056D, 0x02B6, 0x093B, 0x049B, 0x0655, 0x06A9,
        0x0754, 0x0B6A, 0x056C, 0x0AAD, 0x0555, 0x0B29, 0x0B92, 0x0BA9, 0x05D4, 0x0ADA, 0x055A,
        0x0AAB, 0x0595, 0x0749, 0x0764, 0x0BAA, 0x05B5, 0x02B6, 0x0A56, 0x0E4D, 0x0B25, 0x0B52,
        0x0B6A, 0x05AD, 0x02AE, 0x092F, 0x0497, 0x064B, 0x06A5, 0x06AC, 0x0AD6, 0x055D, 0x049D,
        0x0A4D, 0x0D16, 0x0D95, 0x05AA, 0x05B5, 0x02DA, 0x095B, 0x04AD, 0x0595, 0x06CA, 0x06E4,
        0x0AEA, 0x04F5, 0x02B6, 0x0956, 0x0AAA, 0x0B54, 0x0BD2, 0x05D9, 0x02EA, 0x096D, 0x04AD,
        0x0A95, 0x0B4A, 0x0BA5, 0x05B2, 0x09B5, 0x04D6, 0x0A97, 0x0547, 0x0693, 0x0749, 0x0B55,
        0x056A, 0x0A6B, 0x052B, 0x0A8B, 0x0D46, 0x0DA3, 0x05CA, 0x0AD6, 0x04DB, 0x026B, 0x094B,
        0x0AA5, 0x0B52, 0x0B69, 0x0575, 0x0176, 0x08B7, 0x025B, 0x052B, 0x0565, 0x05B4, 0x09DA,
        0x04ED, 0x016D, 0x08B6, 0x0AA6, 0x0D52, 0x0DA9, 0x05D4, 0x0ADA, 0x095B, 0x04AB, 0x0653,
        0x0729, 0x0762, 0x0BA9, 0x05B2, 0x0AB5, 0x0555, 0x0B25, 0x0D92, 0x0EC9, 0x06D2, 0x0AE9,
        0x056B, 0x04AB, 0x0A55, 0x0D29, 0x0D54, 0x0DAA, 0x09B5, 0x04BA, 0x0A3B, 0x049B, 0x0A4D,
        0x0AAA, 0x0AD5, 0x02DA, 0x095D, 0x045E, 0x0A2E, 0x0C9A, 0x0D55, 0x06B2, 0x06B9, 0x04BA,
        0x0A5D, 0x052D, 0x0A95, 0x0B52, 0x0BA8, 0x0BB4, 0x05B9, 0x02DA, 0x095A, 0x0B4A, 0x0DA4,
        0x0ED1, 0x06E8, 0x0B6A, 0x056D, 0x0535, 0x0695, 0x0D4A, 0x0DA8, 0x0DD4, 0x06DA, 0x055B,
        0x029D, 0x062B, 0x0B15, 0x0B4A, 0x0B95, 0x05AA, 0x0AAE, 0x092E, 0x0C8F, 0x0527, 0x0695,
        0x06AA, 0x0AD6, 0x055D, 0x029D,
    ];

    // From https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L264
    const ICU4C_YEAR_START_ESTIMATE_FIX: [i64; 1601 - 1300] = [
        0, 0, -1, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, 1, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, -1, 0, 1, 0,
        0, 0, -1, 0, 1, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, -1, -1, 0, -1, 0, 1, 0, 0, 0, -1, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, -1, -1, 0, -1, 0, 0, -1, -1, 0, -1, 0,
        -1, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, -1, 0, 1, 0, 0, 0, 0, 0, 1, 0,
        1, 0, 0, 0, -1, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
        0, 0, -1, 0, 0, 0, 1, 1, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 1,
        0, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0, -1, 0, 1, 0, 0, 0, -1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0,
        0, -1, 0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, -1, -1, 0, -1, 0, 1, 0, 0, -1, -1, 0, 0, 1,
        1, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
    ];

    let icu4c = ICU4C_ENCODED_MONTH_LENGTHS
        .into_iter()
        .zip(ICU4C_YEAR_START_ESTIMATE_FIX)
        .enumerate()
        .map(
            |(years_since_1300, (encoded_months_lengths, year_start_estimate_fix))| {
                // https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L858
                let month_lengths =
                    core::array::from_fn(|i| (1 << (11 - i)) & encoded_months_lengths != 0);
                // From https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L813
                let year_start = ((354.36720 * years_since_1300 as f64) + 460322.05 + 0.5) as i64
                    + year_start_estimate_fix;
                (
                    1300 + years_since_1300 as i32,
                    PackedHijriYearData::try_new(
                        1300 + years_since_1300 as i32,
                        month_lengths,
                        ISLAMIC_EPOCH_FRIDAY + year_start,
                    )
                    .unwrap(),
                )
            },
        )
        .collect::<Vec<_>>();

    let icu4x = (1300..=1600).zip(DATA.iter().copied()).collect::<Vec<_>>();

    assert_eq!(icu4x, icu4c);
}
