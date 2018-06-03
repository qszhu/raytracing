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
#### single core
width | height | samples/pixel | bounces | time
--- | --- | --- | --- | ---
200 | 100 | 100 | 50 | 1m31s
200 | 100 | 100 | 10 | 1m23s

#### multi-core
width | height | samples/pixel | bounces | time
--- | --- | --- | --- | ---
200 | 100 | 100 | 10 | 36s
