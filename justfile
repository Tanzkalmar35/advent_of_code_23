work day part:
    cargo watch -x "check -p {{day}}" -s "just test -p {{day}}" -s "just lint {{day}}" -s "just bench {{day}} {{part}}" -s "just flamegraph {{day}} {{part}}"
test +FLAGS='-p day-01':
    cargo nextest run {{FLAGS}}
bench-all:
    cargo bench -q > benchmarks.txt
bench day part:
    cargo bench --bench {{day}} {{part}} >> {{day}}.bench.txt
