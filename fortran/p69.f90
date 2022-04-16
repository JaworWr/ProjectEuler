module sieve
    implicit none
    public

    integer(kind=8), parameter :: MAX_VAL = 1000005
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

program p69
    use sieve
    implicit none

    integer(kind=8), parameter :: max = 1000000
    integer(kind=8) :: i, best_i
    integer(kind=8), dimension(:), allocatable :: totients

    call sieve_init()
    print *, totient(int([7, 25, 60, 2*2*3*3*7], kind=8))
    totients = totient([(i, i = 1, max)])
    best_i = 1
    do i = 2, max
        if (i * totients(best_i) > best_i * totients(i)) then
            best_i = i
        end if
    end do
    print *, best_i
end