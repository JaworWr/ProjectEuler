module sieve
    implicit none
    public

    integer(kind=8), parameter :: MAX_VAL = 10000005
    integer :: n_primes
    integer(kind=8), allocatable, dimension(:) :: primes, spf
    logical, allocatable, dimension(:) :: is_prime
contains
    subroutine sieve_init()
        integer :: i, j

        allocate(primes(MAX_VAL), spf(MAX_VAL), is_prime(MAX_VAL))
        n_primes = 0
        is_prime = .true.
        is_prime(1) = .false.
        spf(1) = 1

        do i = 2, MAX_VAL
            if (is_prime(i)) then
                n_primes = n_primes + 1
                primes(n_primes) = i
                spf(i) = i
            end if

            do j = 1, n_primes
                if (i * primes(j) > MAX_VAL .or. primes(j) > spf(i)) then
                    exit
                end if
                is_prime(i * primes(j)) = .false.
                spf(i * primes(j)) = primes(j)
            end do
        end do
    end

    pure function dpf(n)
        ! distinct prime factors
        integer(kind=8), value :: n
        integer(kind=8) :: n_factors, last_factor, factor
        integer(kind=8), dimension(:), allocatable :: dpf

        n_factors = 0
        last_factor = -1
        ! at most log_2(n) distinct prime factors
        allocate(dpf(bit_size(n)))

        do while (n > 1)
            factor = spf(n)
            if (factor /= last_factor) then
                last_factor = factor
                n_factors = n_factors + 1
                dpf(n_factors) = factor
            end if
            n = n / factor
        end do
        dpf = dpf(:n_factors)
    end

    elemental function totient(n)
        integer(kind=8), value :: n
        integer(kind=8) :: totient, i
        integer(kind=8), dimension(:), allocatable :: factors

        if (n == 1) then
            totient = 1
            return
        end if

        factors = dpf(n)
        totient = n
        do i = 1, size(factors)
            totient = totient * (factors(i) - 1) / factors(i)
        end do
    end
end

module p70_funcs
    use stdlib_sorting
    implicit none
contains
    pure function revdigits(n)
        integer(kind=8), value :: n
        integer :: i
        integer(kind=8), dimension(:), allocatable :: revdigits

        if (n == 0) then
            revdigits = [0]
            return
        end if

        allocate(revdigits(bit_size(n)))
        i = 0
        do while (n > 0)
            i = i + 1
            revdigits(i) = modulo(n, 10)
            n = n / 10
        end do
        revdigits = revdigits(:i)
    end

    pure function is_permutation(a1, a2)
        integer(kind=8), dimension(:), intent(in) :: a1, a2
        integer(kind=8), dimension(:), allocatable :: s1, s2
        logical :: is_permutation

        if (size(a1) /= size(a2)) then
            is_permutation = .false.
            return
        end if

        s1 = a1
        s2 = a2
        call sort(s1)
        call sort(s2)
        is_permutation = all(s1 == s2)
    end

    elemental function digit_permutation(n1, n2) result(ok)
        integer(kind=8), intent(in) :: n1, n2
        integer(kind=8), dimension(:), allocatable :: d1, d2
        logical :: ok
        
        d1 = revdigits(n1)
        d2 = revdigits(n2)
        ok = is_permutation(d1, d2)
    end
end

program p70
    use sieve
    use p70_funcs
    implicit none

    integer(kind=8), parameter :: max = 10000000
    integer :: i, best_i, ok_count
    integer(kind=8), dimension(:), allocatable :: values, totients
    logical, dimension(:), allocatable :: is_ok

    call sieve_init()
    values = [(int(i, kind=8), i = 2, max)]
    totients = totient(values)
    is_ok = digit_permutation(values, totients)
    values = pack(values, is_ok)
    totients = pack(totients, is_ok)
    ok_count = count(is_ok)
    print *, ok_count
    best_i = 1
    do i = 1, ok_count
        if (values(i) * totients(best_i) < values(best_i) * totients(i)) then
            best_i = i
        end if
    end do
    print *, best_i, values(best_i), totients(best_i)
end