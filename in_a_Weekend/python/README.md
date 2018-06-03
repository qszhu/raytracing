### setup
* (optional) PyPy
```bash
$ virtualenv -p pypy env
$ . env/bin/activate
```
* `$ pip install tqdm`

### running time
width | height | samples/pixel | bounces | time
--- | --- | --- | --- | ---
200 | 100 | 100 | 50 | 1m31s
200 | 100 | 100 | 10 | 1m23s
