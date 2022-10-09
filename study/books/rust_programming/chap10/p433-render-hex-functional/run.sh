cargo run -- $(
	echo 'Rust in Action' |
		sha256sum |
		cut -f1 -d' '
	)
ls
cat *.svg
echo
firefox *.svg
