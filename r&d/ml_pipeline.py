# from paddleocr import PaddleOCR  
from model2vec import StaticModel
import numpy as np
from sklearn.metrics import pairwise_distances
import onnx
import pypdfium2 as pdm
import cv2
import os
from rapidfuzz import process

"""

TODO: 
    -find new embedder to account for words with similar definitions
    -reduce paddleOCR and embedder to ONNX model
        -prune/distill each
"""


def pdf2vec(filename: str) -> np.array:
    """
    Takes a pdf filename and converts it to an rgb array.

    Args:
        pdf (str): file name of pdf with n pages

    Returns:
        np array: n dimensional array of image arrays, with the i-th element being H(i) x W(i) x 3
    """
    
    pdf = pdm.PdfDocument(filename)
    
    pages = []

    for _, page in enumerate(pdf): #iterate through each page in pdf
        bitmap = page.render(scale=300/72) #render page at 300 DPI
        img = bitmap.to_numpy()[:, :, :3] 

        # img = cv2.cvtColor(img, cv2.COLOR_RGB2GRAY) #rgb to grayscale
        # img = cv2.fastNlMeansDenoising(img) #denoises
        # img = cv2.adaptiveThreshold(img, 255, cv2.ADAPTIVE_THRESH_MEAN_C, cv2.THRESH_BINARY, 31, 10) #clarifies image
        # img = img[..., np.newaxis] #for ocr

        pages.append(img)
    
    pdf.close()

    return pages

def compute_similarities(query: np.ndarray, embeddings : np.ndarray) -> tuple[np.ndarray, np.ndarray]:
    """
    Takes a list of text embeddings and compares each to a tokenized search query (cosine similarity). 

    Args:
        query (np array): tokenized query
        embeddings (np array): tokenized texts

    Returns:
        first np array: list of indices sorted by similarity scores.
        second np array: list of similarity scores for each text.
    """

    distances = pairwise_distances(query, embeddings, metric='cosine')[0]
    most_similar_indices = np.argsort(distances) #lower scores are better

    return most_similar_indices, distances

def top_matches(query: str, words: list[str], n_matches: int = 5) -> list[str]:
    """
    Given a query and a list of words, does a fuzzy search and returns the top n matches.

    Args:
        query (str): word to search for
        words (str): list of words to search 
        n_matches (int): number of matches to retrieve (default 5)

    Returns:
        list[str]: ordered list of top n matches in the word list
    """

    scores = process.extract(query, words, limit = n_matches)
    top_scores = [scores[i][0] for i in range(n_matches)] #retrieves the top n words
    return top_scores

directory = "./medicinal_files/"
files = os.listdir(directory)[2:]
texts = []

embedder = StaticModel.from_pretrained("minishlab/potion-multilingual-128M")
# ocr = PaddleOCR(
#     use_doc_orientation_classify=False, # Disables document orientation classification model via this parameter
#     use_doc_unwarping=False, # Disables text image rectification model via this parameter
#     use_textline_orientation=False, # Disables text line orientation classification model via this parameter
# )


# for filename in files:
#     text = ""
    
#     vec_file = pdf2vec("".join((directory, filename)))
    
#     for page in vec_file:
#         text += ' '.join(ocr.predict(page)[0]["rec_texts"]) + "\n"
#     texts.append( text )

# output = '\n\n\n'.join(texts)

with open("readings.txt", 'r') as out:
    texts = out.read().split("\n\n\n")

with open("words.txt", 'r') as dictionary:
    words = dictionary.read().split("\n")

embeddings = embedder.encode(texts)
search = "allergy"
closest_queries = top_matches(search, words)
query_embeds = []

for query in closest_queries: #testing sensitivity of embedder
    token = embedder.encode(query)[None, :]
    print(token.shape)
    indices, _ = compute_similarities(token, embeddings)
    print(query)
    print([files[i] for i in indices])

# indices, values = compute_similarities(test, embeddings)
# print([files[i] for i in indices])
# print([values[i] for i in indices])
