Benchmarks
==========


Tokenization
------------

Following benchmarks illustrate the tokenization accuracy (F1 score) on `UD treebanks <https://universaldependencies.org/>`_
,

======= ========= =========  =========== ======= 
  lang   dataset   regexp     spacy 2.1   vtext            
======= ========= =========  =========== ======= 
  en     EWT        0.812     0.972       0.966   
  en     GUM        0.881     0.989       0.996   
  de     GSD        0.896     0.944       0.964   
  fr     Sequoia    0.844     0.968       0.971   
======= ========= =========  =========== ======= 

and the English tokenization speed in million words per second (MWPS)

================== ========== =========== ==========
 .                   regexp     spacy 2.1   vtext
================== ========== =========== ==========
 **Speed (MB/s)**   3.1 MWPS   0.14 MWPS   2.1 MWPS
================== ========== =========== ==========


Text vectorization
------------------

Below are  benchmarks for converting
textual data to a sparse document-term matrix using the 20 newsgroups dataset, 
run on Intel(R) Xeon(R) CPU E3-1270 v6 @ 3.80GHz,

 ===============================  =====================  =================== =================
  Speed (MB/s)                     scikit-learn 0.20.1     vtext (n_jobs=1)  vtext (n_jobs=4)
 ===============================  =====================  =================== =================
  CountVectorizer.fit              14                  104               225
  CountVectorizer.transform        14                  82                303
  CountVectorizer.fit_transform    14                  70                NA
  HashingVectorizer.transform      19                  89                309
 ===============================  =====================  =================== =================

Note however that these two estimators in vtext currently support only a fraction of
scikit-learn's functionality.
