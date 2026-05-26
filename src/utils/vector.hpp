namespace fe {
    namespace utils {


        /**
         * @brief Representation of any Vector of type T of N dimensions.
         */
        template <typename T, uint16_t N>
        class Vector {

        private:
            std::array<T, N> content; // Store 1xN matrix of T type
        public:
            Vector(/* args */);
            ~Vector() = default;

            T normalize();

            T[N] get
        };
    } // namespace utils
    
} // namespace fe



