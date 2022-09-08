export function insertionSort(array) {
  for (let i in array) {
    let elem = array[i];
    let j = i;
    while (j > 0 && array[j - 1] > elem) {
      array[j] = array[j - 1];
      j = j - 1;
    }
    array[j] = elem;
  }
  return array;
}
