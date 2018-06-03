### setup
* (optional) PyPy
```bash
$ sudo apt install pypy pypy-dev
$ virtualenv -p pypy env
$ . env/bin/activate
```
* `$ pip install tqdm pillow`

### run
```bash
python main.py out.png
```

### running time
width | height | samples/pixel | bounces | time | workers
--- | --- | --- | --- | --- | ---
200 | 100 | 100 | 50 | 47s | 1
200 | 100 | 100 | 10 | 45s | 1
200 | 100 | 100 | 10 | 24s | 2
200 | 100 | 100 | 10 | 15s | 4
200 | 100 | 100 | 10 | 19s | 8
