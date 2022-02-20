module sieve
    implicit none
    public

    integer, parameter :: MAX_VAL = 1000000
    integer :: n_primes
    integer, allocatable, dimension(:) :: primes, spf
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
end

module numeric
    implicit none
contains
    elemental function gcd(a, b)
        integer(kind=8), value :: a, b
        integer(kind=8) :: gcd, tmp

        do while (b /= 0)
            tmp = a
            a = b
            b = modulo(tmp, b)
        end do
        gcd = a
    end

    elemental function lcm(a, b)
        integer(kind=8), value :: a, b
        integer(kind=8) :: lcm

        lcm = a / gcd(a, b) * b
    end
end

module p752_funcs
    implicit none
    public

    integer(kind=8), parameter :: M(2, 2) = reshape([1, 1, 7, 1], [2, 2])
    integer(kind=8), parameter :: S(2) = [1, 0]
contains
    pure function matpow(a, n, m) result(r)
        integer(kind=8), dimension(:, :), intent(in) :: a
        integer(kind=8), value :: n, m
        integer(kind=8), dimension(:, :), allocatable :: b, r
        integer :: i

        if (n < 0) error stop "n must not be negative"

        allocate(b, mold=a)
        allocate(r, mold=a)
        b = modulo(a, m)
        r = 0
        do i = 1, size(a, 1)
            r(i, i) = 1
        end do

        do while (n > 0) 
            if (modulo(n, 2) == 1) then
                r = modulo(matmul(r, b), m)
            end if
            b = modulo(matmul(b, b), m)
            n = n / 2
        end do
    end

    pure function check_g(p, x) result(ok)
        integer(kind=8), value :: p, x
        logical :: ok
        integer(kind=8), allocatable, dimension(:, :) :: m1

        m1 = matpow(M, x, p)
        ok = all(matmul(m1, S) == S)
    end

    elemental function g_prime(p) result(g)
        integer(kind=8), value :: p
        integer(kind=8) :: g
        integer(kind=8) :: p2, i

        p2 = p**2 - 1
        g = p2
        if (p == 2 .or. p == 3) then
            g = 0
            return
        else if (p == 7) then
            g = 7
            return
        end if

        do i = 2, p - 1
            if (modulo(p2, i) /= 0) then
                cycle
            end if

            if (check_g(p, i)) then
                g = i
                return
            end if

            if (check_g(p, p2 / i)) then
                g = p2 / i
            end if
        end do
    end
end

program p752
    use p752_funcs
    use sieve
    implicit none

    integer(kind=8), allocatable, dimension(:) :: g_results, tmp
    integer :: i

    call sieve_init()

    allocate(g_results(MAX_VAL))
    g_results = 0

    tmp = g_prime(int(primes(1:n_primes), 8))
    do i = 1, n_primes
        g_results(primes(i)) = tmp(i)
    end do

    do i = 2, MAX_VAL
        g_results(i) = g(int(i, 8))
    end do
    
    print *, sum(g_results)

contains
    pure function g(x)
        use numeric
        integer(kind=8), value :: x
        integer(kind=8) :: g
        integer(kind=8) :: p, c, r

        p = 0
        c = 0
        g = 1
        do while (x > 1)
            if (spf(x) /= p) then
                if (p /= 0) then
                    r = g_results(p) * p**(c - 1)
                    g = lcm(g, r)
                end if
                p = spf(x)
                c = 0
            end if
            c = c + 1
            x = x / spf(x)
        end do
        r = g_results(p) * p**(c - 1)
        g = lcm(g, r)
    end
end
