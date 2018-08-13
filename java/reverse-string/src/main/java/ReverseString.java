class ReverseString {

    String reverse(String inputString) {
        StringBuilder reverse = new StringBuilder();
				reverse.append(inputString);
				return reverse.reverse().toString();
    }
  
}